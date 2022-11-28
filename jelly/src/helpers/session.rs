use std::env;
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;

#[cfg(not(feature = "production"))]
pub fn create_session()->SessionMiddleware<CookieSessionStore>{
    let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(key.as_bytes())
    )
    .cookie_name("sessionid".into())
    .cookie_secure(false)
    .cookie_path("/".into())
    .build()
}

#[cfg(feature = "production")]
pub fn create_session()->SessionMiddleware<CookieSessionStore>{
    // !production needs no domain set, because browsers.
    let domain = env::var("SESSIONID_DOMAIN").expect("SESSIONID_DOMAIN not set!");
    let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");
    SessionMiddleware::builder(
        CookieSessionStore::default(), key.as_bytes()
    )
    .cookie_name("sessionid".into())
    .cookie_secure(true)
    .cookie_path("/".into())
    .cookie_domain(Some(domain))
    // .cookie_same_site(SameSite::Lax)
    // .cookie_http_only(false)
    // .session_length(SessionLength::BrowserSession {
    //     state_ttl: Some(Duration::new(259200, 259200)),
    // })
    .build()
}