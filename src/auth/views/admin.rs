use std::env;

use jelly::Result;
use jelly::prelude::*;
use jelly::actix_web::web::Form;
use jelly::guards::AuthSessionName;
use jelly::guards::csrf::extractor::Csrf;
use jelly::actix_session::SessionExt as _;

use super::create_context;
use crate::models::admin::Admin;
use crate::auth::forms::AdminLoginForm;


/// Show admin login form
pub async fn show_login_form(request: HttpRequest) -> Result<HttpResponse> {
    let session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_default();
    let auth_session_name = AuthSessionName(session_name);
    if !request.is_guest(&auth_session_name)? {
        return request.redirect("/admin/dashboard/");
    }
    // let db = request.db_pool()?;
    // Admin::create_admin(db).await?;
    request.render(200, "auth/admin/login.html", Context::new())
}

/// POST-handler for admin login.
pub async fn login(request: HttpRequest, form: Csrf<Form<AdminLoginForm>>) -> Result<HttpResponse> {
    let session_name = env::var("ADMIN_SESSION_NAME").unwrap_or_default();
    let auth_session_name = AuthSessionName(session_name);
    if !request.is_guest(&auth_session_name)? {
        return request.redirect("/admin/dashboard/");
    }

    let mut form = form.into_inner().into_inner();
    if !form.is_valid() {
        let context = create_context("Invalid email or password.", &form);
        return request.render(400, "auth/admin/login.html", context);
    }

    let db = request.db_pool()?;
    if let Ok(user) = Admin::authenticate(&form, db).await {
        request.set_user(&auth_session_name, user)?;
        return request.redirect("/admin/dashboard/");
    }

    let context = create_context("Incorrect email or password.", &form);
    request.render(400, "auth/admin/login.html", context)
}

/// POST-handler for admin logout.
pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    let session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_default();
    request.get_session().remove(session_name.as_str());
    request.redirect("/auth/login/admin/")
}
