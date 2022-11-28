//! A custom Error type, along with a custom Result wrapper, that we use for
//! returning responses. This module handles converting several differing
//! error formats into the one we use for responding.

use crate::error::template::render;
use actix_web::{HttpResponse, ResponseError};
use actix_session::{SessionGetError, SessionInsertError};
use std::{error, fmt, env::VarError, io::Error as IOError};
use crate::guards::auth::auth_config::MissingAuthSessionError;
use crate::guards::csrf::{CsrfError, extractor::CsrfExtractorError};

/// This enum represents the largest classes of errors we can expect to
/// encounter in the lifespan of our application. Feel free to add to this
/// as necessary; `Generic()` exists for anything further in the stack that
/// might not fit here by default.
#[derive(Debug)]
pub enum Error {
    ActixWeb(actix_web::error::Error),
    Anyhow(anyhow::Error),
    Database(sqlx::Error),
    Generic(String),
    Template(tera::Error),
    Json(serde_json::error::Error),
    Radix(radix::RadixErr),
    InvalidPassword,
    InvalidAccountToken,
    PasswordHasher(djangohashers::HasherError),
    CsrfToken(CsrfExtractorError<CsrfError>),
    MissingAuthSessionError(MissingAuthSessionError),
    EnvError(VarError),
    IOError(IOError),
    SessionGetError(SessionGetError),
    SessionInsertError(SessionInsertError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ActixWeb(e) => Some(e),
            Error::Anyhow(e) => Some(e.root_cause()),
            Error::Database(e) => Some(e),
            Error::Template(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Radix(e) => Some(e),
            Error::MissingAuthSessionError(e) => Some(e),
            Error::EnvError(e) => Some(e),
            Error::IOError(e) => Some(e),
            Error::SessionGetError(e) => Some(e),
            Error::SessionInsertError(e) => Some(e),
            Error::CsrfToken(e) => Some(
                match e {
                    CsrfExtractorError::InvalidToken => &CsrfError::TokenMismatch,
                    CsrfExtractorError::Inner(e) => e,
                }
            ),

            Error::Generic(_)
            | Error::InvalidPassword
            | Error::InvalidAccountToken
            | Error::PasswordHasher(_) => None,
        }
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(e: actix_web::error::Error) -> Self {
        Error::ActixWeb(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Json(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Anyhow(e)
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Error::Template(e)
    }
}

impl From<radix::RadixErr> for Error {
    fn from(e: radix::RadixErr) -> Self {
        Error::Radix(e)
    }
}

impl From<djangohashers::HasherError> for Error {
    fn from(e: djangohashers::HasherError) -> Self {
        Error::PasswordHasher(e)
    }
}

impl From<CsrfExtractorError<CsrfError>> for Error {
    fn from(e: CsrfExtractorError<CsrfError>) -> Self {
        Error::CsrfToken(e)
    }
}

impl From<MissingAuthSessionError> for Error {
    fn from(e: MissingAuthSessionError) -> Self {
        Error::MissingAuthSessionError(e)
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::EnvError(e)
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::IOError(e)
    }
}

impl From<SessionGetError> for Error {
    fn from(e: SessionGetError) -> Self {
        Error::SessionGetError(e)
    }
}

impl From<SessionInsertError> for Error {
    fn from(e: SessionInsertError) -> Self {
        Error::SessionInsertError(e)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(render(self))
    }
}
