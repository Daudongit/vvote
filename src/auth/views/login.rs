use std::env;

use jelly::Result;
use jelly::prelude::*;
use jelly::actix_web::web::Form;
use jelly::guards::AuthSessionName;
use jelly::guards::csrf::extractor::Csrf;
use jelly::actix_session::SessionExt as _;

use crate::models::voter::Voter;
use crate::auth::forms::ValidCsrfForm as CsrfForm;

type ValidCsrfForm = Csrf<Form<CsrfForm>>;

/// Show voter login form
pub async fn show_login_form(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "auth/login.html", Context::new())
}

/// login voter post handler
pub async fn login(request: HttpRequest, _: ValidCsrfForm) -> Result<HttpResponse> {
    let session_name = 
        env::var("VOTER_SESSION_NAME").unwrap_or_default();
    let auth_config_name = AuthSessionName(session_name);
    if !request.is_guest(&auth_config_name)? {
        return request.redirect("/elections/");
    }

    let ip_error = Error::Generic("Valid ip address is needed to vote".into());
    let voter_ip = request.peer_addr().ok_or(ip_error)?.ip().to_string();
    let db = request.db_pool()?;
    let mut voter = 
        Voter::find_by_ip(&voter_ip, db).await;
    if let Err(_) = voter {
        voter = Voter::create(&voter_ip, db).await;
    }

    if let Ok(user) = voter {
        request.set_user(&auth_config_name, user)?;
        return request.redirect("/elections/");
    }
    request.flash("error", "Unable to login.")?;
    request.redirect("/")
}

/// login voter logout handler
pub async fn logout(request: HttpRequest) -> Result<HttpResponse> {
    let session_name = 
        env::var("VOTER_SESSION_NAME").unwrap_or_default();
    request.get_session().remove(session_name.as_str());
    request.redirect("/")
}
