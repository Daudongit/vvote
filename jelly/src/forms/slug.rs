use serde::{Deserialize, Deserializer, Serialize};
use super::{Validation, ParseStringError};
use std::str::FromStr;
use std::ops::Deref;
use std::fmt;


/// A field for validating that a URL slug is valid for a URL.
#[derive(Debug, Default, Serialize)]
pub struct SlugField {
    pub value: String,
    pub errors: Vec<String>,
}

impl fmt::Display for SlugField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for SlugField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| SlugField {
            value: t,
            errors: Vec::new(),
        })
    }
}

impl Deref for SlugField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for SlugField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new() }
    }
}

impl FromStr for SlugField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new() })
    }
}

impl Validation for SlugField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        if self.value == "" {
            self.errors.push(format!("{field_name} cannot be blank!"));
        }

        if self.value.contains(" ") {
            self.errors.push(format!("Slugs({field_name}) can't contain spaces."));
        }

        self.errors.len() == 0
    }
}
