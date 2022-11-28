//! Your Service Description here, etc.
extern crate simple_excel_writer as excel;
use std::io;

pub mod admin;
pub mod auth;
pub mod front;
pub mod models;
pub mod csrf;
pub mod extractor;

use jelly::Server;
use csrf::csrf_routes;


pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_csrf_routes(csrf_routes())
        .register_service(auth::home_configure)
        .register_service(admin::configure)
        .register_service(front::configure)
        .register_service(auth::configure)
        .run()
        .await?
        .await
}


// use std::{io, env};
// use auth::config::{ADMIN_AUTH_ROUTES, AUTH_ROUTES, SetAuthRoutes as _};

// pub async fn main() -> io::Result<()> {
//     let stdout = io::stdout();
//     let _lock = stdout.lock();
//     let auth_routes = Box::new(AUTH_ROUTES);
//     let admin_auth_routes = Box::new(ADMIN_AUTH_ROUTES);
//     let voter_session_name = 
//         env::var("VOTER_SESSION_NAME").unwrap_or_else(|_| "".into());
//     let admin_session_name = 
//         env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
//     Server::new()
//         .register_csrf_routes(csrf_routes())
//         .set_auth_guard(voter_session_name.as_str(), "/", auth_routes)
//         .set_auth_guard(admin_session_name.as_str(), "/admin/", admin_auth_routes)
//         .register_service(admin::configure)
//         .register_service(front::configure)
//         .register_service(auth::configure)
//         // .register_jobs(accounts::jobs::configure)
//         .run()
//         .await?
//         .await
// }
