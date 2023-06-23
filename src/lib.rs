//! Your Service Description here, etc.

pub mod admin;
pub mod auth;
pub mod front;
pub mod models;
pub mod error;
pub mod helpers;
pub mod extractors;

use std::io;

use jelly::Server;
use helpers::csrf::csrf_routes;
extern crate simple_excel_writer as excel;


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
