//! Front module
pub mod views;
pub mod forms;

use std::cell::RefCell;

use jelly::guards::Auth;
use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

use crate::auth::config::auth_config;

pub fn configure_results(config: &mut ServiceConfig) {
    let auth = Auth{auth_config: RefCell::new(Some(auth_config()))};
    config.service(
        scope("/elections").wrap(auth)
        .service(
            resource("/")
                .route(get().to(views::election::index)).name("fornt.elections.index"),
        )
        .service(
            resource("/results/check/{election}/")
                .route(get().to(views::result::show)).name("front.result.show")
        ) // need auth
        .service(
            resource("/results/")
                .route(get().to(views::result::index)).name("front.result.index"),
        )// need auth
        .service(
            resource("/store/")
                .route(post().to(views::election::store)).name("front.elections.store"),
        ) // need auth
        .service(
            resource("/show/")
                .route(post().to(views::election::show)).name("front.elections.show"),
        )// need auth
    );
}
