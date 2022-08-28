use jelly::error::Error;
use jelly::sqlx::{self, postgres::PgPool};

#[derive(Debug)]
pub struct Nominee{}

impl Nominee{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(id) as nominee_count from nominees")
        .fetch_one(pool).await?.nominee_count.unwrap_or(0))
    }
}