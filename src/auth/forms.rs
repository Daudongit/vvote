use jelly::serde::{Deserialize, Serialize};
use jelly::forms::{EmailField, PasswordField, Validation};
use jelly::guards::csrf::extractor::{CsrfGuarded, CsrfToken};

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminLoginForm {
    pub csrf: CsrfToken,
    pub email: EmailField,
    pub password: PasswordField
}

impl Validation for AdminLoginForm {
    fn is_valid(&mut self) -> bool {
        self.email.is_valid_field("email") && 
        !self.password.is_emty_value("password")
    }
}

impl CsrfGuarded for AdminLoginForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}

#[derive(Debug, Deserialize)]
pub struct ValidCsrfForm {
    pub csrf: CsrfToken
}

impl CsrfGuarded for ValidCsrfForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf
    }
}
