use std::{error, fmt};

use reqwest::Error as ReqwestError;
use serde_json::error::Error as SerdeError;
use jelly::error::error::Error as JellyError; 

#[derive(Debug)]
pub enum Error {
    ReqwestError(ReqwestError),
    JellyError(JellyError),
    Json(SerdeError),
    Generic(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ReqwestError(e) => Some(e),
            Error::JellyError(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Generic(_) => None
        }
    }
}

impl From<JellyError> for Error {
    fn from(e: JellyError) -> Self {
        Error::JellyError(e)
    }
}

impl Into<JellyError> for Error {
    fn into(self) -> JellyError {
        match self {
            Error::JellyError(e) => e,
            Error::ReqwestError(e) => JellyError::Generic(e.to_string()),
            Error::Json(e) => JellyError::Generic(e.to_string()), 
            Error::Generic(e) => JellyError::Generic(e.to_string())
        }
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Error::Json(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Error::ReqwestError(e)
    }
}
