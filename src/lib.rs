//! Your Service Description here, etc.

use std::io;

// #[macro_use]
// extern crate log;

pub mod admin;
pub mod auth;
pub mod front;
pub mod models;

use jelly::Server;

pub async fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let _lock = stdout.lock();

    Server::new()
        .register_service(admin::configure)
        .register_service(front::configure)
        // .register_jobs(accounts::jobs::configure)
        .run()
        .await?
        .await
}
