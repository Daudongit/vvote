pub mod slot;
pub mod voter;
pub mod admin;
pub mod result;
pub mod nominee;
pub mod election;
pub mod position;

use jelly::serde::Serialize;
use jelly::error::error::Error;
use jelly::sqlx::{self, Row as _, postgres::PgPool};

#[derive(Debug, Serialize)]
pub struct PaginatedEntity<T> {
    current_page: u16,
    data: Vec<T>,
    per_page: u8,
    total: u64
}

impl<T> From<(u16, u64, Vec<T>)> for PaginatedEntity<T>{
    fn from(entity: (u16, u64, Vec<T>)) -> Self {
        let (current_page, total, data) = entity;
        let items_per_page = items_per_page();
        Self {current_page, data, total, per_page: items_per_page}
    }    
}

pub async fn get_aggregate_count(table: &str, pool: &PgPool)->Result<i64, Error>{
    let sql = format!("select count(id) as aggregate from {}", table);
    let row = sqlx::query(&sql).fetch_one(pool).await?;
    let aggregate = row.try_get("aggregate").unwrap_or(0); 
    Ok(aggregate)
}

pub fn items_per_page()->u8{
    let default = String::from("10");
    let items_per_page = std::env::var("PAGINATION_SIZE").unwrap_or(default);
    items_per_page.parse().unwrap_or(10)
}

pub fn calculate_pagination_offset(page: u16)->Result<(u16, u8), Error>{
    let items_per_page = items_per_page();
    Ok((((page - 1) * items_per_page as u16), items_per_page))
    // Ok((((page - 1) * items_per_page as u16 + 1), items_per_page))
}