use std::fmt;
use log::error;
use std::ops::Deref;
use std::str::FromStr;
use chrono::NaiveDate;
use super::{Validation, ParseStringError};
use serde::{Serialize, Deserialize, Deserializer};

/// A field for accepting and validating a date string.
#[derive(Debug, Default, Serialize)]
pub struct DateField {
    pub value: String,
    pub date: Option<chrono::NaiveDate>,
    pub errors: Vec<String>,
}

impl fmt::Display for DateField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for DateField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| DateField {
            value: t,
            date: None,
            errors: Vec::new(),
        })
    }
}

impl Deref for DateField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for DateField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new(), date: None }
    }
}

impl FromStr for DateField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new(), date: None })
    }
}

impl Validation for DateField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        // match NaiveDate::parse_from_str(&self.value, "%m/%d/%Y") {
        match NaiveDate::parse_from_str(&self.value, "%Y/%m/%d") {
            Ok(date) => {
                self.date = Some(date);
                true
            }

            Err(e) => {
                error!("Error parsing DateField: {}", e);
                self.errors.push(format!("Invalid date({field_name}) format."));
                false
            }
        }
    }
}
