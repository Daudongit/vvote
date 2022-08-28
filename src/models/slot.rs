use jelly::error::Error;
use jelly::sqlx::{self, postgres::PgPool};

#[derive(Debug)]
pub struct Slot{}

impl Slot{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        let row = sqlx::query!("select count(id) as slot_count from slots")
        .fetch_one(pool).await?;
        Ok(row.slot_count.unwrap_or(0))
    }
}