//! Your Service Description here, etc.
extern crate simple_excel_writer as excel;
use std::io;

pub mod admin;
pub mod auth;
pub mod front;
pub mod models;
pub mod error;
pub mod helpers;
pub mod extractors;

use jelly::Server;
use helpers::csrf::csrf_routes;


pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_csrf_routes(csrf_routes())
        .register_service(auth::configure_home)
        .register_service(admin::configure)
        // .register_service(front::configure)
        .register_service(front::configure_results)
        .register_service(auth::configure)
        .run()
        .await?
        .await
}
