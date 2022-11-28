pub mod auth_config;

use std::task::{Context, Poll};

use std::cell::RefCell;
use crate::error::render;
use auth_config::AuthConfig;
use crate::request::Authentication;
use actix_web::{Error, HttpResponse};
use actix_web::http::header::LOCATION;
use actix_service::{Service, Transform};
use futures::future::{ok, Either, Ready};
use actix_web::dev::{ServiceRequest, ServiceResponse};


/// A guard that enables route and scope authentication gating.
#[derive(Debug, Clone)]
pub struct Auth {
    /// list of authenticated routes and redirect path
    /// Authentication configuration
    pub auth_config: RefCell<Option<AuthConfig>>
}

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let auth_config = 
            self.auth_config.borrow_mut().take().unwrap_or(vec![]);
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
    auth_config: Option<Vec<AuthConfig>>,

    /// The service provided.
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let auth_config = self.auth_config.take().unwrap_or(vec![]);
        let (request, payload) = req.into_parts();
        let status = request.is_authenticated(auth_config);

        match status {
            Ok((is_authorized, _)) if is_authorized == true => {
                let req = ServiceRequest::from_parts(request, payload).ok().unwrap();
                Either::Left(self.service.call(req))
            }

            Ok((_, redirect_to)) => Either::Right(ok(ServiceResponse::new(
                request,
                HttpResponse::Found()
                    .header(LOCATION, redirect_to)
                    .finish()
                    .into_body(),
            ))),

            Err(e) => Either::Right(ok(ServiceResponse::new(
                request,
                HttpResponse::InternalServerError()
                    .body(&render(e))
                    .into_body(),
            ))),
        }
    }
}
