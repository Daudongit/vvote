use sqlx::postgres::{PgPoolOptions, Postgres};
use crate::helpers::templates;
use std::sync::{Arc, RwLock};
use tera::Tera;
use sqlx::Pool;
use std::env;

type Configurations = (String, String, Arc<RwLock<Tera>>, Pool<Postgres>);

pub async fn get_config_datas()-> Configurations{
    let bind = env::var("BIND_TO").expect("BIND_TO not set!");
    let _root_domain = env::var("JELLY_DOMAIN").expect("JELLY_DOMAIN not set!");

    let template_store = templates::load();
    let templates = template_store.templates.clone();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let pool = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Unable to connect to database!");
    (bind, _root_domain, templates, pool)
}
