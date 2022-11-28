use futures::TryStreamExt;
use jelly::forms::DateField;
use std::collections::HashMap;
use jelly::error::error::Error;
use jelly::chrono::NaiveDateTime;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, QueryBuilder, Postgres ,postgres::PgPool, Row as _};
use crate::admin::forms::{ElectionForm, ToggleElectionForm, ElectionVisibilityForm};

type VisibilityForm = ElectionVisibilityForm;
type ResultMap = Result<HashMap<i32, Slot>, Error>;
type ResultDate = Result<(NaiveDateTime, NaiveDateTime), Error>;

#[derive(Deserialize)]
pub struct RequestQParam{
    pub page: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedElection {
    current_page: u16,
    data: (Vec<Election>, HashMap<i32,Slot>),
    per_page: u8,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Election {
    pub id: i32,
    pub title: String,
    pub status: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub can_see_result: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub results_count: i64,
    // slots: Vec<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    pub id: i32,
    pub position_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub pivot: Pivot
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pivot {
    pub election_id: i32,
    pub slot_id: i32
}

impl Election{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(*) as election_count from elections")
        .fetch_one(pool).await?.election_count.unwrap_or(0))
    }

    pub async fn with_slot_paginate(page: u16, pool: &PgPool)->Result<PaginatedElection, Error>{
        let items_per_page = super::items_per_page();
        let total_election_count = super::get_aggregate_count("elections", pool).await?;
        let (election_ids, elections) = Self::get_paginated_record(page, pool).await?;
        let election_slots = Self::get_election_slots(election_ids, pool).await?;
        Ok(
            PaginatedElection{
                current_page:page,
                data:(elections, election_slots),
                per_page:items_per_page,
                total:total_election_count as u64,
            }
        )
    }

    pub async fn get_paginated_record(page: u16, pool: &PgPool)->Result<(Vec<i32>,Vec<Election>), Error>{
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let sql:String = format!("select elections.*, (
            select count(*) from results where elections.id = results.election_id
        ) as results_count from elections limit {} offset {}", items_per_page, offset);
        // let sql = Box::leak(sql.into_boxed_str()); // String to &'static str to live for ever
        let (election_ids, elections) = Self::get_processed_record(sql, pool).await?;
        Ok((election_ids, elections))
    }

    pub async fn get_election_slots(election_ids: Vec<i32>, pool: &PgPool)->Result<HashMap<i32,Slot>, Error>{
        // let election_ids = Box::leak(election_ids.into_boxed_slice()); // for longer live
        let sql: String = "
            select slots.*, election_slot.election_id as pivot_election_id, election_slot.slot_id as pivot_slot_id 
            from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id = ANY($1)
        ".into();

        let slots = Self::process_slot_record(sql, election_ids, pool).await?;
        Ok(slots)
    }

    pub async fn get_processed_record(sql: String, pool: &PgPool)->Result<(Vec<i32>,Vec<Election>), Error>{
        let mut election_ids: Vec<i32> = vec![];
        let mut elections: Vec<Election> = vec![];
        let mut rows = sqlx::query(sql.as_str()).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            let id = row.try_get("id")?;
            election_ids.push(id as i32);
            elections.push(
                Election{
                    id,
                    title: row.try_get("title")?,
                    status: row.try_get("status")?,
                    start: row.try_get("start")?,
                    end: row.try_get("end")?,
                    can_see_result: row.try_get("can_see_result")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    results_count: row.try_get("results_count")?
                }
            );
        }
        Ok((election_ids, elections))
    }

    pub async fn process_slot_record(sql: String, election_ids: Vec<i32>, pool: &PgPool)->ResultMap{
        let mut slots: HashMap<i32, Slot> = HashMap::new();
        let mut slot_rows = sqlx::query(sql.as_str())
        .bind(&election_ids[..]).fetch(pool);
        while let Some(row) = slot_rows.try_next().await? {
            let election_id = row.try_get("pivot_election_id")?;
            let value = Slot{
                id: row.try_get("id")?,
                position_id: row.try_get("position_id")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                pivot: Pivot { 
                    election_id, 
                    slot_id: row.try_get("pivot_slot_id")?
                }
            };
           slots.insert(election_id, value);
        }
        Ok(slots)
    }

    pub async fn create(form: &mut ElectionForm, pool: &PgPool) -> Result<i32, Error> {
        let (start_date, end_date) = 
            Self::format_dates(&mut form.start_date, &mut form.end_date)?;
        let mut tx = pool.begin().await?;
        let election_id = sqlx::query!(
            "insert into elections (title, start, \"end\", created_at, updated_at) 
            values ($1, $2, $3, now(), now()) returning id",
            form.title.value, start_date, end_date
        ).fetch_one(&mut tx).await?.id;
        let mut query_builder = 
            Self::election_slot_qb(form.slot.clone(), election_id);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(election_id)
    }

    pub async fn update(form: &mut ElectionForm, id: i32, pool: &PgPool) -> Result<(), Error> {
        let (start_date, end_date) = 
            Self::format_dates(&mut form.start_date, &mut form.end_date)?;
        let mut tx = pool.begin().await?;
        sqlx::query!(
            "update elections set title=$1, start=$2, \"end\"=$3, updated_at=now() where id=$4", 
            form.title.value, start_date, end_date, id
        ).execute(&mut tx).await?;
        sqlx::query!("delete from election_slot where election_id=$1", id)
        .execute(&mut tx).await?;
        let mut query_builder = 
            Self::election_slot_qb(form.slot.clone(), id);
        query_builder.build().execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<(), Error> {
        let mut tx = pool.begin().await?;
        sqlx::query!("delete from results where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from election_slot where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from ipvalidations where election_id=$1", id)
        .execute(&mut tx).await?;
        sqlx::query!("delete from elections where id=$1", id)
        .execute(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn lock_unlock(id: i32, form: &ToggleElectionForm, pool: &PgPool)->Result<i32, Error>{
        let status = if form.status == 1 {0}else{1};
        sqlx::query!("update elections set status=$1, updated_at=now() where id=$2", status, id)
        .execute(pool).await?;
        Ok(status)
    }

    pub async fn set_visibility(id: i32, form: &VisibilityForm, pool: &PgPool)->Result<bool, Error>{
        let can_see_toggle = !form.can_see_result;
        sqlx::query!(
            "update elections set can_see_result=$1, updated_at=now() where id=$2",
            can_see_toggle, id
        )
        .execute(pool).await?;
        Ok(can_see_toggle)
    }

    fn election_slot_qb<'a>(slots: Vec<u64>, id: i32)->QueryBuilder<'a, Postgres>{
        let mut query_builder = QueryBuilder::new(
            "insert into election_slot (slot_id, election_id, created_at, updated_at) "
        );
        query_builder.push_values(
            slots, |mut b, slot| {
            b.push_bind(slot as i64)
                .push_bind(id)
                .push_bind("now()")    
                .push_bind("now()");
        });
        query_builder
    }

    fn format_dates(start_date: &mut DateField, end_date: &mut DateField) ->ResultDate {
        let start_date = start_date.date.take()
            .ok_or_else(|| Error::Generic("Invalid start date".into()))?
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::Generic("Invalid start time".into()))?;
        let end_date = end_date.date.take()
            .ok_or_else(|| Error::Generic("Invalid end date".into()))?
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::Generic("Invalid end time".into()))?;
        Ok((start_date, end_date))
    }
}
