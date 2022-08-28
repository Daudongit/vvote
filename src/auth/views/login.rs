use actix_session::UserSession;
use jelly::prelude::*;
use jelly::Result;

/// Show admin login form
pub async fn show_login_form(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "auth/login.html", {
        let context = Context::new();
        context
    })
}

/// login admin view
pub async fn login(request: HttpRequest) -> Result<HttpResponse> {
    //let user = request.user()?;

    request.redirect("admin/dashboard")
}

pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    request.get_session().clear();
    request.redirect("/")
}