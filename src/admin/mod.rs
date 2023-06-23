//! Admin module.
pub mod views;
pub mod forms;

use std::cell::RefCell;

use jelly::guards::Auth;
use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};

use crate::auth::views::admin;
use crate::auth::config::admin_auth_config;

pub fn configure(config: &mut ServiceConfig) {
    let auth = Auth{auth_config: RefCell::new(Some(admin_auth_config()))};
    config.service(
        scope("/admin").wrap(auth)
            .service(
                resource("/")
                    .route(get().to(admin::show_login_form)).name("admin.index"),
            )
            .service(
                resource("/dashboard/")
                    .route(get().to(views::dashboard::index)).name("admin.dashboard"),
            )
            .service(
                resource("/results/")
                    .route(get().to(views::result::index)).name("admin.results.index"),
            )
            .service(
                resource("/results/{election}/")
                    .route(get().to(views::result::show)).name("admin.results.show"),
            )
            .service(
                resource("/suspend/{election}/")
                    .route(post().to(views::suspend::update)).name("admin.suspend.update"),
            )//http patch method not supported for form by actix
            .service(
                resource("report/visibility/{election}/")
                    .route(post().to(views::visibility::update)).name("admin.visibility.update"),
            )//http patch method not supported for form by actix
            .service(
                resource("/export/vote/{election}/{position}/")
                    .route(get().to(views::export::index)).name("admin.export.index"),
            ) // query param {type?}
            .service(
                resource("/export/{election}/{slot}/")
                    .route(get().to(views::export::show)).name("admin.export.show"),
            ) // query param {type?}

            //nominees
            .service(
                resource("/nominees/")
                    .route(get().to(views::nominee::index)).name("admin.nominees.index"),
            ) 
            .service(
                resource("/nominees/store/")
                    .route(post().to(views::nominee::store)).name("admin.nominees.store"),
            ) 
            .service(
                resource("/nominees/{id}/")
                    .route(post().to(views::nominee::update)).name("admin.nominees.update"),
            )//http put method not supported for form by actix yet
            .service(
                resource("/nominees/{id}/delete/") 
                    .route(post().to(views::nominee::destroy)).name("admin.nominees.delete"),
            ) //http delete method not supported for form by actix yet

            //elections
            .service(
                resource("/elections/")
                    .route(get().to(views::election::index)).name("admin.elections.index"),
            ) 
            .service(
                resource("/elections/store/")
                    .route(post().to(views::election::store)).name("admin.elections.store"),
            ) 
            .service(
                resource("/elections/{id}/")
                    .route(post().to(views::election::update)).name("admin.elections.update"),
            )
            .service(
                resource("/elections/{id}/delete/")
                    .route(post().to(views::election::destroy)).name("admin.elections.delete"),
            )

            //positions
            .service(
                resource("/positions/")
                    .route(get().to(views::position::index)).name("admin.positions.index"),
            ) 
            .service(
                resource("/positions/store/")
                    .route(post().to(views::position::store)).name("admin.positions.store"),
            ) 
            .service(
                resource("/positions/{id}/")
                    .route(post().to(views::position::update)).name("admin.positions.update"),
            )
            .service(
                resource("/positions/{id}/delete/")
                    .route(post().to(views::position::destroy)).name("admin.positions.delete"),
            )

            //slots
            .service(
                resource("/slots/")
                    .route(get().to(views::slot::index)).name("admin.slots.index"),
            ) 
            .service(
                resource("/slots/store/")
                    .route(post().to(views::slot::store)).name("admin.slots.store"),
            ) 
            .service(
                resource("/slots/{id}/")
                    .route(post().to(views::slot::update)).name("admin.slots.update"),
            )
            .service(
                resource("/slots/{id}/delete/")
                    .route(post().to(views::slot::destroy)).name("admin.slots.delete"),
            )
    );
}
