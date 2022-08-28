//! Auth module.

use jelly::actix_web::web::{get, post, resource, scope, ServiceConfig};
// use jelly::guards::Auth;

pub mod views;
pub mod forms;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/admin/")
            .service(
                resource("login")
                    .route(get().to(views::login::show_login_form))
                    .route(post().to(views::login::login)),
            ),
            // .service(resource("logout").route(post().to(views::login::logout)),),
    );
}
