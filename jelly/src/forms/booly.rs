use serde::{Deserialize, Deserializer, Serialize};
use std::str::{FromStr, ParseBoolError};
use super::Validation;
use std::ops::Deref;
use std::fmt;


/// A simple BoolField.
///
/// Checks to see if the value is `true` or not in validation. This means
/// that your input should literally pass `true` or `false`.
#[derive(Debug, Default, Serialize)]
pub struct BoolField {
    pub value: bool,
    pub errors: Vec<String>,
}

impl fmt::Display for BoolField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for BoolField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| BoolField {
            value: t,
            errors: Vec::new(),
        })
    }
}

impl Deref for BoolField {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for BoolField {
    fn from(text: String)->Self{
        Self { value: text.parse::<bool>().is_ok(), errors: Vec::new() }
    }
}

impl FromStr for BoolField {
    type Err = ParseBoolError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.parse()?, errors: Vec::new() })
    }
}

impl Validation for BoolField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        if !self.value {
            self.errors.push(format!("Bad boolean({field_name}) value?"));
            return false;
        }
        true
    }
}
