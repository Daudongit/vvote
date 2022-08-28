use jelly::error::Error;
use jelly::sqlx::{self, Row, postgres::PgPool};

#[derive(Debug)]
pub struct Results{}

impl Results{
    pub async fn vote_count(pool: &PgPool)->Result<i64, Error>{
        let sub_sql = "
            select voter_id, election_id, position_id, string_agg(id::text, ',') as ids 
            from results 
            group by voter_id, election_id, position_id
        ";
        let sql = format!("select count(voter_id) as vote_count from ({}) as results_count", &sub_sql);
        Ok(sqlx::query(&sql).fetch_one(pool).await?.get("vote_count"))
    }
}

