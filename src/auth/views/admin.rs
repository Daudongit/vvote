use jelly::actix_session::SessionExt as _;
use jelly::guards::csrf::extractor::Csrf;
use crate::auth::forms::AdminLoginForm;
use jelly::guards::AuthSessionName;
use crate::models::admin::Admin;
use jelly::actix_web::web::Form;
use jelly::prelude::*;
use jelly::Result;
use std::env;


/// Show admin login form
pub async fn show_login_form(request: HttpRequest) -> Result<HttpResponse> {
    let session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
    let auth_session_name = AuthSessionName(session_name);
    if !request.is_guest(&auth_session_name)? {
        return request.redirect("/admin/dashboard/");
    }
    // let db = request.db_pool()?;
    // Admin::create_admin(db).await?;
    request.render(200, "auth/admin/login.html", {
        Context::new()
    })
}

/// POST-handler for admin login.
pub async fn login(request: HttpRequest, form: Csrf<Form<AdminLoginForm>>) -> Result<HttpResponse> {
    let session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
    let auth_session_name = AuthSessionName(session_name);
    if !request.is_guest(&auth_session_name)? {
        return request.redirect("/admin/dashboard/");
    }

    let mut form = form.into_inner();
    if !form.is_valid() {
        return request.render(400, "auth/admin/login.html", {
            let mut context = Context::new();
            context.insert("error", "Invalid email or password.");
            context.insert("form", &form.into_inner());
            context
        });
    }

    let db = request.db_pool()?;
    if let Ok(user) = Admin::authenticate(&form, db).await {
        request.set_user(&auth_session_name, user)?;
        return request.redirect("/admin/dashboard/");
    }

    request.render(400, "auth/admin/login.html", {
        let mut context = Context::new();
        context.insert("error", "Invalid email or password.");
        context.insert("form", &form.into_inner());
        context
    })
}

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    let session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
    request.get_session().remove(session_name.as_str());
    request.redirect("/login/admin/")
}