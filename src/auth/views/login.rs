use jelly::actix_session::SessionExt as _;
use jelly::prelude::*;
use jelly::Result;

/// Show admin login form
pub async fn show_login_form(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "auth/login.html", {
        Context::new()
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