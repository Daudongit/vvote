use std::pin::Pin;
use jelly::error::Error;
use std::collections::HashMap;
use jelly::chrono::{NaiveDateTime};
use futures::{Stream, TryStreamExt};
use jelly::sqlx::Error as SqlxError;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, Row, postgres::{PgPool, PgRow}};


type PinRow = Pin<Box<dyn Stream<Item = Result<PgRow, SqlxError>> + std::marker::Send>>;
// Pin<Box<dyn futures::Stream<Item = Result<Election::get_election_slots::{closure#0}::Record, jelly::sqlx::Error>> + std::marker::Send>>

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
    id: i32,
    title: String,
    status: i32,
    start: NaiveDateTime,
    end: NaiveDateTime,
    can_see_result: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    results_count: i64,
    // slots: Vec<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    id: i32,
    position_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    pivot: Pivot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pivot {
    election_id: i32,
    slot_id: i32,
}

impl Election{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(*) as election_count from elections;")
        .fetch_one(pool).await?.election_count.unwrap_or(0))
    }

    pub async fn get_with_slot_paginated(page: u16, pool: &PgPool)->Result<PaginatedElection, Error>{
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
        ) as results_count from elections limit {} offset {}", items_per_page, offset).to_owned();
        let sql = Box::leak(sql.into_boxed_str()); // String to &'static str to live for ever
        let mut rows = sqlx::query(sql).fetch(pool);
        let (election_ids, elections) = Self::process_record(&mut rows).await?;
        Ok((election_ids, elections))
    }

    pub async fn get_election_slots(election_ids: Vec<i32>, pool: &PgPool)->Result<HashMap<i32,Slot>, Error>{
        let election_ids = Box::leak(election_ids.into_boxed_slice()); // for longer live
        let mut slot_rows = sqlx::query(
            "select slots.*, election_slot.election_id as pivot_election_id, election_slot.slot_id as pivot_slot_id 
            from slots
            inner join election_slot on slots.id = election_slot.slot_id
            where election_slot.election_id = ANY($1)
        ").bind(&election_ids[..]).fetch(pool);
        let slots = Self::process_slot_record(&mut slot_rows).await?;
        Ok(slots)
    }

    pub async fn process_record(rows: &mut PinRow)->Result<(Vec<i32>,Vec<Election>), Error>{
        let mut election_ids: Vec<i32> = vec![];
        let mut elections: Vec<Election> = vec![];
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

    pub async fn process_slot_record(rows: &mut PinRow)->Result<HashMap<i32,Slot>, Error>{
        let mut slots: HashMap<i32, Slot> = HashMap::new();
        while let Some(row) = rows.try_next().await? {
            let election_id = row.try_get("pivot_election_id")?;
            let value = Slot{
                id: row.try_get("id")?,
                position_id: row.try_get("position_id")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                pivot: Pivot { 
                    election_id: election_id, 
                    slot_id: row.try_get("pivot_slot_id")?
                }
            };
           slots.insert(election_id, value);
        }
        Ok(slots)
    }
}








// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

// use serde_derive;

// #[derive(Serialize, Deserialize)]
// pub struct Welcome {
//     current_page: i64,
//     data: Vec<Datum>,
//     from: i64,
//     last_page: i64,
//     next_page_url: Option<serde_json::Value>,
//     path: String,
//     per_page: i64,
//     prev_page_url: Option<serde_json::Value>,
//     to: i64,
//     total: i64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Datum {
//     id: i64,
//     title: String,
//     status: i64,
//     start: String,
//     end: String,
//     can_see_result: bool,
//     created_at: String,
//     updated_at: String,
//     results_count: i64,
//     slots: Vec<Slot>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Slot {
//     id: i64,
//     position_id: i64,
//     created_at: String,
//     updated_at: String,
//     pivot: Pivot,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Pivot {
//     election_id: i64,
//     slot_id: i64,
// }


// https://stackoverflow.com/questions/26984409/bootstrap-pagination-using-php-mysql