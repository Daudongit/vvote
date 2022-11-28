//! Implements some framework-level pieces, primarily useful in debugging scenarios.

use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::ServiceConfig;
use actix_web::http::Method;
use actix_web::Result;
use tera::Context;

use crate::error::error::Error;
use crate::request::Render;

/// default service handler (for 404 and others)
pub async fn default_service_handler(request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.method() == Method::GET {
        return not_found(request).await
    }
    Ok(HttpResponse::MethodNotAllowed().finish())
}

/// Shorthand method for throwing a big ol' 404.
#[inline(always)]
pub async fn not_found(request: HttpRequest) -> Result<HttpResponse, Error> {
    request.render(404, "errors/404.html", Context::new())
}

/// Enables serving static files.
#[cfg(feature = "static")]
pub fn static_handler(config: &mut ServiceConfig) {
    let static_path =
        std::env::var("STATIC_ROOT").expect("Running in debug without STATIC_ROOT set!");

    let fs = actix_files::Files::new("/static", &static_path);
    config.service(fs);
}

/// A noop static handler for production usage.
#[cfg(not(feature = "static"))]
pub fn static_handler(_config: &mut ServiceConfig) {}

