use std::{env, fs};

use jelly::serde::Serialize;
use jelly::error::error::Error;
use jelly::chrono::NaiveDateTime;
use actix_easy_multipart::tempfile::Tempfile;
use jelly::sqlx::{self, postgres::{PgPool, PgRow}, FromRow, Row as _};

use crate::models::PaginatedEntity;
use crate::admin::forms::MultipartNomineeForm;
#[cfg(not(feature = "cloudinary"))]
use crate::helpers::upload::local::LocalFileUpload as _;
#[cfg(feature = "cloudinary")]
use crate::helpers::upload::cloudinary::CloudinaryFileUpload as _;

type ResultNomineePaginated = Result<PaginatedEntity<Nominee>, Error>;
type NomineePayload<'a> = (&'a MultipartNomineeForm, &'a Option<String>);

#[derive(Debug, Serialize)]
pub struct Nominee{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub image: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl FromRow<'_, PgRow> for Nominee {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            email: row.try_get("email")?,
            image: row.try_get("image")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?
        })
    }
}

impl Nominee{
    pub async fn count(pool: &PgPool)->Result<i64, Error>{
        Ok(sqlx::query!("select count(id) as nominee_count from nominees")
        .fetch_one(pool).await?.nominee_count.unwrap_or(0))
    }

    pub async fn get_paginated_record(page: u16, pool: &PgPool)->ResultNomineePaginated{
        let (offset, items_per_page) = super::calculate_pagination_offset(page)?;
        let total_nominee_count = super::get_aggregate_count("nominees", pool).await?;
        let sql = format!(
            "select * from nominees order by created_at, id asc limit {} offset {}",
            items_per_page, offset
        );
        let nominees: Vec<Nominee> = 
            sqlx::query_as(sql.as_str()).fetch_all(pool).await?;
        Ok((page, total_nominee_count as u64, nominees).into())
    }

    pub async fn save_upload_image(image_file: Option<Tempfile>)->Result<Option<String>, Error>{
        if let Some(image) = image_file {
            if image.size > 0 && image.file_name.is_some() {
                return image.persist_file("nominee_".into()).await;
                // .map_err(Into::into);
            }
        }
        Ok(None)
    }
    
    pub async fn create(nominee_payload: NomineePayload<'_>, pool: &PgPool)->Result<i32, Error>{
        let (form, image_name) = nominee_payload;
        let id = sqlx::query!(
            "insert into nominees
            (first_name, last_name, email, image, description, created_at, updated_at)
            values ($1, $2, $3, $4, $5, now(), now()) returning id",
            form.first_name.value, form.last_name.value, form.email.value,
            image_name.as_ref().map(|s|s.as_str()).unwrap_or_default(), 
            form.description.value
        ).fetch_one(pool).await?.id;
        Ok(id)
    }

    pub async fn unlink_prev_image(image_url: Option<String>) -> Result<(), Error> {
        if let Some(image_path) = image_url {
            if !image_path.starts_with("http") {
                let upload_path = env::var("UPLOAD_PATH")?;
                if let Err(err) = fs::remove_file(upload_path +"/"+ &image_path) {
                    dbg!("========== Unable to save file =================", err);
                }
            }
        }
        Ok(())
    }

    pub async fn update(nominee_payload: NomineePayload<'_>, id: i32, pool: &PgPool)->Result<(),Error>{
        let (form, image_name) = nominee_payload;
        if image_name.is_none() {
            sqlx::query!(
                "update nominees 
                set first_name=$1, last_name=$2, email=$3, description=$4, updated_at=now() 
                where id=$5",
                form.first_name.value, form.last_name.value, form.email.value,
                form.description.value, id
            )
            .execute(pool).await?;
        } else {
            sqlx::query!(
                "update nominees 
                set first_name=$1, last_name=$2, email=$3, image=$4, description=$5, updated_at=now() 
                where id=$6",
                form.first_name.value, form.last_name.value, form.email.value,
                image_name.as_ref().unwrap(), form.description.value, id
            )
            .execute(pool).await?;
        }
        Ok(())
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<bool, Error> {
        let mut is_deleted = false;
        let nominee_count = sqlx::query!(
            "select count(*) as nominee_count from nominee_slot where nominee_id=$1", id
        ).fetch_one(pool).await?.nominee_count;
        if let Some(count) = nominee_count {
            if count == 0 {
                sqlx::query!("delete from nominees where id=$1", id)
                .execute(pool).await?;
                is_deleted = true;
            }
        }
        Ok(is_deleted)
    }
}


