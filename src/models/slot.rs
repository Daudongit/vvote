use futures::TryStreamExt;
use std::collections::HashMap;
use jelly::error::error::Error;
use jelly::chrono::NaiveDateTime;
use crate::admin::forms::SlotForm;
use serde::{Serialize, Deserialize};
use crate::models::nominee::Nominee;
use jelly::sqlx::{self, QueryBuilder, Postgres, postgres::PgPool, Row as _};

type ResultMap = Result<HashMap<i32, Nominee>, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedSlot {
    current_page: u16,
    data: (Vec<Slot>, HashMap<i32, Nominee>),
    per_page: u8,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    pub id: i32,
    pub position_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Slot{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        let row = sqlx::query!("select count(id) as slot_count from slots")
        .fetch_one(pool).await?;
        Ok(row.slot_count.unwrap_or(0))
    }

    pub async fn with_nominee_paginate(page: u16, pool: &PgPool)->Result<PaginatedSlot, Error>{
        let items_per_page = super::items_per_page();
        let total_slot_count = super::get_aggregate_count("slots", pool).await?;
        let (slot_ids, slots) = Self::paginate(page, pool).await?;
        let slot_nominee = Self::slot_nominees(slot_ids, pool).await?;
        Ok(
            PaginatedSlot{
                current_page:page,
                data:(slots, slot_nominee),
                per_page:items_per_page,
                total:total_slot_count as u64,
            }
        )
    }

    pub async fn paginate(page: u16, pool: &PgPool)->Result<(Vec<i32>, Vec<Slot>), Error>{
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let mut slots: Vec<Slot> = vec![];
        let mut slot_ids: Vec<i32> = vec![];
        let mut rows =
            sqlx::query!(
                "select * from slots order by updated_at desc limit $1 offset $2",
                items_per_page as i64, offset as i64
            ).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            let err_msg = String::from("Unformated datetime.");
            let id = row.id;
            let created_at = 
                row.created_at.ok_or_else(||Error::Generic(err_msg.clone()))?;
            let updated_at = row.updated_at.ok_or(Error::Generic(err_msg))?;
            slot_ids.push(id);
            slots.push(Slot{id, created_at, updated_at, position_id: row.position_id});
        }
        Ok((slot_ids, slots))
    }

    pub async fn slot_nominees(slot_ids: Vec<i32>, pool: &PgPool)->ResultMap{
        let mut nominees: HashMap<i32, Nominee> = HashMap::new();
        let mut nominee_rows = sqlx::query("
            select nominees.*, nominee_slot.nominee_id as pivot_nominee_id, 
            nominee_slot.slot_id as pivot_slot_id 
            from nominees
            inner join nominee_slot on nominees.id = nominee_slot.slot_id
            where nominee_slot.slot_id = ANY($1)
        ").bind(&slot_ids[..]).fetch(pool);
        while let Some(row) = nominee_rows.try_next().await? {
            let slot_id = row.try_get("pivot_slot_id")?;
            nominees.insert(
                slot_id,
                Nominee{
                    id: row.try_get("id")?,
                    first_name: row.try_get("first_name")?,
                    last_name: row.try_get("last_name")?,
                    email: row.try_get("email")?,
                    image: row.try_get("image")?,
                    description: row.try_get("description")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?
                }
            );
        }
        Ok(nominees)
    }

    pub async fn create(form: &SlotForm, pool: &PgPool) -> Result<i32, Error> {
        let mut tx = pool.begin().await?;
        let slot_id = sqlx::query!(
            "insert into slots (position_id, created_at, updated_at) 
            values ($1, now(), now()) returning id",
            form.position_id as i64
        ).fetch_one(&mut tx).await?.id;
        let mut query_builder = 
            Self::nominee_slot_qb(form.nominees.clone(), slot_id);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(slot_id)
    }

    pub async fn update(form: &SlotForm, id: i32, pool: &PgPool) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        sqlx::query!(
            "update slots set position_id=$1, updated_at=now() where id=$2", 
            form.position_id as i32, id
        ).execute(&mut tx).await?;
        sqlx::query!("delete from nominee_slot where slot_id=$1", id)
        .execute(&mut tx).await?;
        let mut query_builder = 
            Self::nominee_slot_qb(form.nominees.clone(), id);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool, Error> {
        let mut is_deleted = false;
        let election_count = sqlx::query!(
            "select count(*) as election_count from election_slot where slot_id=$1", id
        ).fetch_one(pool).await?.election_count;
        if let Some(count) = election_count {
            if count == 0 {
                sqlx::query!("delete from slots where id=$1", id)
                .execute(pool).await?;
                is_deleted = true;
            }
        }
        Ok(is_deleted)
    }

    fn nominee_slot_qb<'a>(nominees: Vec<u64>, id: i32)->QueryBuilder<'a, Postgres>{
        let mut query_builder = QueryBuilder::new(
            "insert into nominee_slot (slot_id, nominee_id, created_at, updated_at) "
        );
        query_builder.push_values(
            nominees, |mut b, nominee| {
            b.push_bind(nominee as i64)
                .push_bind(id)
                .push_bind("now()")    
                .push_bind("now()");
        });
        query_builder
    }
}