use std::env;
use std::sync::{Arc, RwLock};

use serde::Serialize;
use tera::{Context, Tera};
use crate::accounts::User;
use crate::error::error::Error;
use crate::guards::AuthSessionName;
use actix_web::http::header::LOCATION;
use actix_web::{HttpRequest, HttpResponse};
use crate::request::{Authentication, FlashMessages, CsrfTokenization as _};
// use crate::request::{Authentication, FlashMessages};

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
        let data: Option<&Arc<RwLock<Tera>>> = self.app_data();

        // We pull the user and flash messages for all requests;
        // it's blank if a User is anonymous (not authenticated).
        let auth_session_name = 
            AuthSessionName::from_request_sync(self)
            .unwrap_or(AuthSessionName("".into()));
        let user = 
            self.user(&auth_session_name).unwrap_or(User::default());
        let messages = self.get_flash_messages()?;
        let mut host: String = self.uri().host().unwrap_or("").into();
        let path_query = match self.uri().path_and_query(){
            Some(path_query)=>path_query.as_str(),
            _=>""
        };
        if let Some(port) = self.uri().port(){
            if !port.as_str().contains("80") && !host.is_empty() {
                host = format!("{}:{}", host, port.as_str());
            }
        }
        let csrf_token: String = self.get_csrf_token().map_or_else(
            |_|"".into(), |f|f.get().into()
        );
        context.insert("csrf_token", &csrf_token);
        context.insert("user", &user);
        context.insert("flash_messages", &messages);
        context.insert("current_path", &self.path());
        context.insert(
            "full_url", &format!("{}{}", host, path_query)
        );
        if let Some(form_data) = self.get_flash_form()? {
            context.insert("flash_form", &form_data);
        }
        for (k, v) in env::vars() {
            if k.starts_with("JELLY_") {
                context.insert(k, &v);
            }
        }

        if let Some(eng) = data {
            let engine = eng.read().map_err(|e| {
                Error::Generic(format!("Error acquiring template read lock: {:?}", e))
            })?;

            let body = engine.render(template, &context).map_err(Error::from)?;


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
