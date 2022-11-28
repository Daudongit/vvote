use serde::{Deserialize, Deserializer, Serialize};
use super::{Validation, ParseStringError};
use std::str::FromStr;
use std::ops::Deref;
use zxcvbn::zxcvbn;
use std::fmt;


/// A field for validating password strength. Will also include
/// hints on how to make a better password.
#[derive(Debug, Default, Serialize)]
pub struct PasswordField {
    pub value: String,
    pub errors: Vec<String>,
    pub hints: Vec<String>,
}

impl PasswordField {
    pub fn validate_with(&mut self, user_inputs: &[&str], field_name: &str) -> bool {
        if self.value == "" {
            self.errors.push(format!("{field_name} cannot be blank."));
            return false;
        }

        // The unwrap is safe, as it only errors if the
        // password is blank, which we already
        // handle above.
        let estimate = zxcvbn(&self.value, user_inputs).unwrap();
        if estimate.score() <= 2 {
            if let Some(feedback) = estimate.feedback() {
                if let Some(warning) = feedback.warning() {
                    self.errors.push(format!("{}", warning));
                } else {
                    self.errors.push(format!("{}", "Password not strong enough."));
                }

                self.hints = feedback
                    .suggestions()
                    .iter()
                    .map(|s| format!("{}", s))
                    .collect();
            }

            return false;
        }

        true
    }
}

impl fmt::Display for PasswordField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for PasswordField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| PasswordField {
            value: t,
            errors: Vec::new(),
            hints: Vec::new(),
        })
    }
}

impl Deref for PasswordField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for PasswordField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new(), hints: Vec::new() }
    }
}

impl FromStr for PasswordField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new(), hints: Vec::new()})
    }
}

impl Validation for PasswordField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        self.validate_with(&[], field_name)
    }
}
