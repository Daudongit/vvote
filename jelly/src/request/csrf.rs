use actix_web::HttpRequest;
use actix_web::HttpMessage as _;
use crate::guards::csrf::{CsrfError, extractor::CsrfToken};

pub trait CsrfTokenization {
    /// Returns generated csrf.
    fn get_csrf_token(&self) -> Result<CsrfToken, CsrfError>;
}

impl CsrfTokenization for HttpRequest {
    fn get_csrf_token(&self) -> Result<CsrfToken, CsrfError> {
        self.extensions().get().cloned().ok_or(CsrfError::MissingToken)
    }
}
