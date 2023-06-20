use std::env;
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
#[cfg(feature = "redis-session")]
use actix_session::storage::RedisSessionStore;
#[cfg(feature = "cookie-session")]
use actix_session::storage::CookieSessionStore;
#[cfg(feature = "redis-actor-session")]
use actix_session::storage::RedisActorSessionStore;

fn redis_connection_key()->(String, String){
    let redis_connection = 
        env::var("REDIS_CONNECTION_STRING")
        .expect("REDIS_CONNECTION_STRING not set!");
    let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");
    (redis_connection, key)
}

#[cfg(feature = "redis-actor-session")]
pub async fn create_session_store_key()->(RedisActorSessionStore, Key){
    let (redis_connection_key, key) = redis_connection_key();
    let store = RedisActorSessionStore::new(redis_connection_key);
    (store, Key::from(key.as_bytes()))
}

#[cfg(feature = "redis-session")]
pub async fn create_session_store_key()->(RedisSessionStore, Key){
    let (redis_connection_key, key) = redis_connection_key();
    let store = 
       match RedisSessionStore::new(redis_connection_key).await {
            Ok(store) => store,
            Err(err) => {
                dbg!(err);
                panic!("Unable to establish redis session")  
            } 
       };
    (store, Key::from(key.as_bytes()))
}

#[cfg(feature = "cookie-session")]
pub async fn create_session_store_key()->(CookieSessionStore, Key){
    let (_, key) = redis_connection_key();
    let store = CookieSessionStore::default();
    (store, Key::from(key.as_bytes()))
}

#[cfg(feature = "redis-actor-session")]
pub fn create_session(store: RedisActorSessionStore, key: Key)
    ->SessionMiddleware<RedisActorSessionStore>{
    SessionMiddleware::builder(store, key)
    .cookie_name("sessionid".into())
    .cookie_secure(false)
    .cookie_path("/".into())
    .build()
}

#[cfg(feature = "redis-session")]
pub fn create_session(store: RedisSessionStore, key: Key)
    ->SessionMiddleware<RedisSessionStore>{
    SessionMiddleware::builder(store, key)
    .cookie_name("sessionid".into())
    .cookie_secure(false)
    .cookie_path("/".into())
    .build()
}

#[cfg(feature = "cookie-session")]
pub fn create_session(store: CookieSessionStore, key: Key)
    ->SessionMiddleware<CookieSessionStore>{
    SessionMiddleware::builder(store, key)
    .cookie_name("sessionid".into())
    .cookie_secure(false)
    .cookie_path("/".into())
    .build()
}
