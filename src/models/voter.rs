use std::env;
use jelly::accounts::User;
use jelly::error::error::Error;
use jelly::sqlx::{self, postgres::PgPool};

pub struct Voter;
// pub struct Voter{
//     id: i32,
//     email: String,
//     ip: String,
//     member_id: String,
//     phone: String,
//     name: String,
//     confirmation_token: String
// }

impl Voter {
    pub async fn find_by_ip(ip: &str, pool: &PgPool) -> Result<User, Error> {
        let session_name = 
            env::var("VOTER_SESSION_NAME").unwrap_or_default();
        let voter = 
            sqlx::query!("select * from voters where ip = $1", ip)
            .fetch_one(pool).await?;

        Ok(User {
            id: voter.id, is_admin: false, session_name,
            name: voter.name.unwrap_or_default(), is_anonymous: false
        })
    }

    pub async fn create(ip: &str, pool: &PgPool) -> Result<User, Error> {
        let session_name = 
            env::var("VOTER_SESSION_NAME").unwrap_or_default();
        let email = format!("{ip}@mail.com");
        let id = sqlx::query!(
            "insert into voters (ip, email, created_at, updated_at) 
            values ($1, $2, now(), now()) returning id", ip, email
        ).fetch_one(pool).await?.id;

        Ok(User {
            id, is_admin: false, name: format!("name_{ip}"),
            is_anonymous: false, session_name
        })
    }
}