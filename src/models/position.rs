use jelly::error::Error;
use jelly::sqlx::{self, postgres::PgPool};

#[derive(Debug)]
pub struct Position{}

impl Position{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(id) as position_count from positions")
        .fetch_one(pool).await?.position_count.unwrap_or(0))
    }
}