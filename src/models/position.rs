use futures::TryStreamExt;
use jelly::error::error::Error;
use jelly::chrono::NaiveDateTime;
use crate::models::PaginatedEntity;
use crate::admin::forms::PositonForm;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, postgres::PgPool, Row as _};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position{
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

impl Position{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(id) as position_count from positions")
        .fetch_one(pool).await?.position_count.unwrap_or(0))
    }

    pub async fn paginate(page: u16, pool: &PgPool)->Result<PaginatedEntity<Position>, Error>{
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let total_position_count = super::get_aggregate_count("positions", pool).await?;
        let sql = format!(
            "select * from positions order by updated_at desc limit {} offset {}", 
            items_per_page, offset
        );
        let positions = Self::get_processed_record(sql, pool).await?;
        Ok((page, total_position_count as u64, positions).into())
    }

    pub async fn get_processed_record(sql:String, pool: &PgPool)->Result<Vec<Position>, Error>{
        let mut positions: Vec<Position> = vec![];
        let mut rows =
         sqlx::query(sql.as_str()).fetch(pool);
        while let Some(row) = rows.try_next().await? {
            positions.push(
                Position{
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?
                }
            );
        }
        Ok(positions)
    }

    pub async fn create(form: &PositonForm, pool: &PgPool) -> Result<i32, Error> {
        let id = sqlx::query!(
            "insert into positions (name, created_at, updated_at) 
            values ($1, now(), now()) returning id",
            form.name.value
        ).fetch_one(pool).await?.id;
        Ok(id)
    }

    pub async fn update(form: &PositonForm, id: i32, pool: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            "update positions set name=$1, updated_at=now() where id=$2", form.name.value, id
        )
        .execute(pool).await?;
        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool, Error> {
        let mut is_deleted = false;
        let slot_count = sqlx::query!(
            "select count(*) as slot_count from slots where position_id=$1", id
        ).fetch_one(pool).await?.slot_count;
        if let Some(count) = slot_count {
            if count == 0 {
                sqlx::query!("delete from positions where id=$1", id)
                .execute(pool).await?;
                is_deleted = true;
            }
        }
        Ok(is_deleted)
    }
}
