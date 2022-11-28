use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};
use jelly::forms::{EmailField, PasswordField, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminLoginForm {
    pub csrf: CsrfToken,
    pub email: EmailField,
    pub password: PasswordField
}

impl Validation for AdminLoginForm {
    fn is_valid(&mut self) -> bool {
        self.email.is_valid() && !self.password.value.is_empty()
    }
}

impl CsrfGuarded for AdminLoginForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}