use std::env;
use std::sync::{Arc, RwLock};

use serde::Serialize;
use tera::{Context, Tera};
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};

use crate::accounts::User;
use crate::error::error::Error;
use crate::guards::AuthSessionName;
use crate::request::{Authentication, FlashMessages, CsrfTokenization as _};

/// A trait for making certain types of response handling easier.
pub trait Render {
    /// Shorthand for rendering a template, with a specific HTTP response code.
    fn render(&self, code: usize, template: &str, context: Context) -> Result<HttpResponse, Error>;

    /// Shorthand for returning a JSON payload.
    fn json<S: Serialize>(&self, code: usize, payload: S) -> Result<HttpResponse, Error>;

    /// Handy redirects helper.
    fn redirect(&self, location: &str) -> Result<HttpResponse, Error>;
}

impl Render for HttpRequest {
    fn render(
        &self,
        code: usize,
        template: &str,
        mut context: Context,
    ) -> Result<HttpResponse, Error> {
        // We pull the user and flash messages for all requests;
        // it's blank if a User is anonymous (not authenticated).
        let auth_session_name = 
            AuthSessionName::from_request_sync(self)
            .unwrap_or(AuthSessionName("".into()));
        let user = 
            self.user(&auth_session_name).unwrap_or(User::default());
        context.insert("csrf_token", &csrf_token(self));
        context.insert("user", &user);
        context.insert("flash_messages", &self.get_flash_messages()?);
        context.insert("current_path", &self.path());
        context.insert("full_url", &full_url(self));
        set_optional_context(self, &mut context)?;
        prepare_response(self, &context, code, template)
    }

    fn json<S: Serialize>(&self, code: usize, payload: S) -> Result<HttpResponse, Error> {
        let o = serde_json::to_string(&payload)?;

        Ok(match code {
            200 => HttpResponse::Ok(),
            400 => HttpResponse::BadRequest(),
            404 => HttpResponse::NotFound(),
            _ => HttpResponse::Ok(),
        }
        .content_type("application/json")
        .body(o))
    }

    fn redirect(&self, location: &str) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Found().append_header((LOCATION, location)).finish())
    }
}

// Helpers
fn full_url(request: &HttpRequest)->String{
    let mut host: String = request.uri().host().unwrap_or_default().into();
    let path_query = match request.uri().path_and_query(){
        Some(path_query)=>path_query.as_str(),
        _=>""
    };
    if let Some(port) = request.uri().port(){
        if !port.as_str().contains("80") && !host.is_empty() {
            host = format!("{}:{}", host, port.as_str());
        }
    }
    format!("{}{}", host, path_query)
}

fn csrf_token(request: &HttpRequest)->String{
    request.get_csrf_token().map_or_else(
        |_|"".into(), |f|f.get().into()
    )
}

fn set_optional_context(request: &HttpRequest, context: &mut Context)->Result<(), Error>{
    if let Some(form_data) = request.get_flash_form()? {
        context.insert("flash_form", &form_data);
    }
    for (k, v) in env::vars() {
        if k.starts_with("JELLY_") {
            context.insert(k, &v);
        }
    }
    Ok(())
}

fn prepare_response(request: &HttpRequest, context: &Context, code: usize, template: &str)
    ->Result<HttpResponse, Error>{
    let data: Option<&Arc<RwLock<Tera>>> = request.app_data();
    if let Some(eng) = data {
        let engine = eng.read().map_err(
            |e| {
            Error::Generic(format!("Error acquiring template read lock: {:?}", e))
        })?;
        let body = 
            engine.render(template, context).map_err(Error::from)?;
        Ok(match code {
            200 => HttpResponse::Ok(),
            400 => HttpResponse::BadRequest(),
            404 => HttpResponse::NotFound(),
            _ => HttpResponse::Ok(),
        }
        .content_type("text/html; charset=utf-8")
        .body(body))
    } else {
        Err(Error::Generic(
            "Unable to locate Templates cache".to_string(),
        ))
    }
}
