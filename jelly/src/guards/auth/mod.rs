pub mod auth_config;

use std::cell::RefCell;
use std::task::{Context, Poll};

use auth_config::AuthConfig;
use actix_web::{Error, HttpResponse};
use actix_web::http::header::LOCATION;
use futures::future::{ok, Either, Ready};
use actix_web::dev::{ServiceRequest, ServiceResponse, Service, Transform};

use crate::error::template::render;
use crate::request::Authentication;


/// A guard that enables route and scope authentication gating.
#[derive(Debug, Clone)]
pub struct Auth {
    /// list of authenticated routes and redirect path
    /// Authentication configuration
    pub auth_config: RefCell<Option<AuthConfig>>
}

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let auth_config = 
            self.auth_config.borrow_mut().take().unwrap_or(AuthConfig::default());
        ok(AuthMiddleware {
            service, auth_config: Some(auth_config)
        })
    }
}

/// Middleware for checking user authentication status and redirecting depending
/// on the result. You generally don't need this type, but it needs to be exported
/// for compiler reasons.
pub struct AuthMiddleware<S> {
    /// list of authenticated routes and redirect path
    /// Authentication configuration
    auth_config: Option<AuthConfig>,

    /// The service provided.
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Error = Error;
    type Response = ServiceResponse;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_config = 
            self.auth_config.clone().unwrap_or(AuthConfig::default());
        let (request, payload) = req.into_parts();
        let status = request.is_authenticated(auth_config);

        match status {
            Ok((is_authorized, _)) if is_authorized == true => {
                let req = ServiceRequest::from_parts(request, payload);
                Either::Left(self.service.call(req))
            }

            Ok((_, redirect_to)) => Either::Right(ok(ServiceResponse::new(
                request,
                HttpResponse::Found()
                    .append_header((LOCATION, redirect_to))
                    .finish()
            ))),

            Err(e) => Either::Right(ok(ServiceResponse::new(
                request,
                HttpResponse::InternalServerError()
                    .body(render(e))
            ))),
        }
    }
}
