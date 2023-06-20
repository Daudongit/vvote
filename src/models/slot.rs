use std::collections::HashMap;

use serde::Serialize;
use futures::TryStreamExt;
use jelly::error::error::Error;
use jelly::sqlx::{self, QueryBuilder, Postgres, postgres::PgPool, Row as _};

use crate::admin::forms::SlotForm;
use crate::models::position::Position;

type ResultMap = Result<HashMap<i32, Vec<Nominee>>, Error>;

#[derive(Debug, Serialize)]
pub struct PaginatedSlot {
    current_page: u16,
    data: (Vec<Slot>, HashMap<i32, Vec<Nominee>>),
    per_page: u8,
    total: u64
}

#[derive(PartialEq)]
struct NomineeId{nominee_id: i32}

#[derive(Debug, Serialize)]
pub struct Nominee{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

#[derive(Debug, Serialize)]
pub struct Slot {
    pub id: i32,
    pub position_id: i32,
    pub position_name: String
}

impl Slot{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        let row = sqlx::query!("select count(id) as slot_count from slots")
        .fetch_one(pool).await?;
        Ok(row.slot_count.unwrap_or(0))
    }

    pub async fn positions(pool: &PgPool)->Result<Vec<Position>, Error>{
        let positions = 
            sqlx::query_as!(Position, "select id, name from positions")
        .fetch_all(pool).await?;
        Ok(positions)
    }

    pub async fn nominees(pool: &PgPool)->Result<Vec<Nominee>, Error>{
        let nominees = sqlx::query_as!(
            Nominee, r#"select id, first_name, last_name, 'no-email' as "email!" from nominees"#
        )
        .fetch_all(pool).await?;
        Ok(nominees)
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
                "select id, position_id, (
                    select name from positions where positions.id=slots.position_id
                ) as position_name from slots order by created_at, id asc limit $1 offset $2",
                items_per_page as i64, offset as i64
            ).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            let id = row.id;
            let position_name = 
                row.position_name.ok_or_else(||Error::Generic("Invalid position name".into()))?;
            slot_ids.push(id);
            slots.push(Slot{id, position_id: row.position_id, position_name });
        }
        Ok((slot_ids, slots))
    }

    pub async fn slot_nominees(slot_ids: Vec<i32>, pool: &PgPool)->ResultMap{
        let mut nominees: HashMap<i32, Vec<Nominee>> = HashMap::new();
        let mut nominee_rows = sqlx::query("
            select nominees.id, nominees.first_name, nominees.last_name, nominees.email, 
            nominee_slot.slot_id as pivot_slot_id from nominees
            inner join nominee_slot on nominees.id = nominee_slot.nominee_id
            where nominee_slot.slot_id = any($1)
        ").bind(&slot_ids[..]).fetch(pool);
        while let Some(row) = nominee_rows.try_next().await? {
            let slot_id = row.try_get("pivot_slot_id")?;
            let nominee = Nominee{
                id: row.try_get("id")?,
                first_name: row.try_get("first_name")?,
                last_name: row.try_get("last_name")?,
                email: row.try_get("email")?
            };
            if let Some(slot_nominees) = nominees.get_mut(&slot_id) {
                slot_nominees.push(nominee);
            }else{
                nominees.insert(slot_id, vec![nominee]);
            }
        }
        Ok(nominees)
    }

    pub async fn create(form: &SlotForm, pool: &PgPool) -> Result<i32, Error> {
        let mut tx = pool.begin().await?;
        let slot_id = sqlx::query!(
            "insert into slots (position_id, created_at, updated_at) 
            values ($1, now(), now()) returning id",
            form.position_id.value as i64
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
            form.position_id.value as i32, id
        ).execute(&mut tx).await?;
        sqlx::query!("delete from nominee_slot where slot_id=$1", id)
        .execute(&mut tx).await?;
        if !form.nominees.is_empty() {
            let mut query_builder = 
                Self::nominee_slot_qb(form.nominees.clone(), id);
            query_builder.build().execute(&mut tx).await?;
        }
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

    fn nominee_slot_qb<'a>(nominees: Vec<u64>, slot_id: i32)->QueryBuilder<'a, Postgres>{
        let mut query_builder = QueryBuilder::new(
            "insert into nominee_slot (slot_id, nominee_id, created_at, updated_at) "
        );
        query_builder.push_values(
            nominees, |mut b, nominee| {
            b.push_bind(slot_id)
                .push_bind(nominee as i64)
                .push(" now() ")    
                .push(" now() ");
        });
        query_builder
    }
}