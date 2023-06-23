//! Your Service Description here, etc.

use std::io;

use jelly::actix_web;

#[actix_web::main]
async fn main() -> io::Result<()> {
    mainlib::main().await
}
