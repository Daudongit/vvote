use jelly::djangohashers::{check_password, make_password};
use jelly::sqlx::{self, postgres::PgPool};
use crate::auth::forms::AdminLoginForm;
use jelly::error::error::Error;
use jelly::accounts::User;
use std::env;

pub struct Admin;

impl Admin {
    pub async fn authenticate(form: &AdminLoginForm, pool: &PgPool) -> Result<User, Error> {
        let session_name = 
            env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
        let user = 
            sqlx::query!("select * from users where email = $1", form.email.value)
            .fetch_one(pool).await?;
    
        if !check_password(&form.password, &user.password)? {
            return Err(Error::InvalidPassword);
        }

        Ok(User {
            id: user.id,
            is_admin: true,
            name: user.name,
            is_anonymous: false,
            session_name
        })
    }

    pub async fn create_admin_sample(pool: &PgPool) -> Result<i32, Error> {
        let password = make_password("secret");
        Ok(sqlx::query!("
            insert into users (name, email, password, created_at, updated_at) 
            values ($1, $2, $3, now(), now()) returning id
        ", "test admin", "admin@gmail.com", password
        ).fetch_one(pool).await?.id)
    }
}