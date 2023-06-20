//! Auth module.
use jelly::actix_web::web::{get, post, scope, resource, ServiceConfig};

pub mod views;
pub mod forms;
pub mod config;

pub fn configure_home(config: &mut ServiceConfig) {
    // Front
    config.service(
        resource("/")
            .route(get().to(views::login::show_login_form)).name("login.show")
    );
    config.service(
        resource("/election/login")
            .route(post().to(views::login::login)).name("election.attempt")
    );
    config.service(
        resource("/election/logout")
            .route(post().to(views::login::logout)).name("election.logout")
    );
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/auth")
        .service(
            resource("/login/")
                .route(post().to(views::login::login)).name("login.store"),
        )
        .service(
            resource("/logout/")
            .route(post().to(views::login::logout)).name("logout"),
        )
        // Admin
        .service(
            resource("/login/admin/")
                .route(get().to(views::admin::show_login_form)).name("admin.login.show")
        )
        .service(
            resource("/admin/login/")
                .route(post().to(views::admin::login)).name("admin.login.store"),
        )
        .service(
            resource("/admin/logout/")
            .route(post().to(views::admin::logout)).name("admin.logout"),
        )
    );
}
