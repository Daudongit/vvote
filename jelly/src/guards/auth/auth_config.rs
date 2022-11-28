use std::error::Error;
use std::fmt::Display;
// use actix_web::http::Method;
// use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde::de::{Error as DeError, Visitor};
use actix_web::{HttpRequest, HttpResponse, HttpMessage, ResponseError};

#[derive(Default, Debug, Clone)]
pub struct AuthConfig {
    pub name: AuthSessionName, //sku
    pub redirect_to: String,
    // pub secure_routes: HashSet<(Method, String)>
}

/// Current authentication session name
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AuthSessionName(pub String);

impl Serialize for AuthSessionName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_newtype_struct("Auth session name", &self.0)
    }
}

impl<'de> Deserialize<'de> for AuthSessionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AuthSessionNameVisitor;
        impl<'de> Visitor<'de> for AuthSessionNameVisitor {
            type Value = AuthSessionName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a auth session name")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(AuthSessionName(v.to_owned()))
            }
        }
        deserializer.deserialize_string(AuthSessionNameVisitor)
    }
}

impl AuthSessionName {
    /// Retrieves a reference of the auth session name.
    #[must_use]
    pub fn get(&self) -> &str {
        self.0.as_ref()
    }

    /// Consumes the struct, returning the underlying string.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn from_request_sync(req: &HttpRequest) -> Result<Self, MissingAuthSessionError> {
        req.extensions()
            .get::<Self>()
            .cloned()
            .ok_or(MissingAuthSessionError)
    }
}

impl AsRef<str> for AuthSessionName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub struct MissingAuthSessionError;

impl Display for MissingAuthSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The auth session name is missing")
    }
}

impl ResponseError for MissingAuthSessionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::UnprocessableEntity().finish()
    }
}

impl Error for MissingAuthSessionError {}