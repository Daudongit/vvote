use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use validator::validate_email;
use super::{Validation, ParseStringError};
use serde::{Deserialize, Deserializer, Serialize};


/// A field for validating that an email address is a valid address.
/// Mostly follows Django semantics.
#[derive(Debug, Default, Serialize)]
pub struct EmailField {
    pub value: String,
    pub errors: Vec<String>,
}

impl fmt::Display for EmailField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for EmailField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| EmailField {
            value: t,
            errors: Vec::new(),
        })
    }
}

impl Deref for EmailField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for EmailField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new() }
    }
}

impl FromStr for EmailField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new() })
    }
}

impl Validation for EmailField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        if self.value == "" {
            self.errors
                .push(format!("{field_name} cannot be blank."));
            return false;
        }

        if !validate_email(&self.value) {
            self.errors.push("Invalid email format.".to_string());
            return false;
        }
        true
    }
}
