pub mod election;
pub mod nominee;
pub mod position;
pub mod result;
pub mod slot;
pub mod user;
pub mod voter;

use jelly::error::Error;
use jelly::sqlx::{self, Row, postgres::PgPool};

pub async fn get_aggregate_count(table: &str, pool: &PgPool)->Result<i64, Error>{
    let sql = format!("select count(id) as aggregate from {}", table);
    let row = sqlx::query(&sql).fetch_one(pool).await?;
    let aggregate = row.try_get("aggredate").unwrap_or(0); 
    Ok(aggregate)
}

pub fn items_per_page()->u8{
    let default = String::from("10");
    let items_per_page = std::env::var("PAGINATION_SIZE").unwrap_or(default);
    items_per_page.parse().unwrap_or(10)
}

pub fn calculate_pagination_offset(page: u16)->Result<(u16, u8), Error>{
    // start = (page - 1) * itemsPerPage + 1
    let items_per_page = items_per_page();
    Ok((((page - 1) * items_per_page as u16 + 1), items_per_page))
}