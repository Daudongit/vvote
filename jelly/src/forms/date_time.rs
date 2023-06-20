use std::fmt;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

use log::error;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize, Deserializer};

use super::{Validation, ParseStringError};

/// A field for accepting and validating a datetime string.
#[derive(Debug, Default, Serialize)]
pub struct DateTimeField {
    pub value: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub errors: Vec<String>,
}

impl fmt::Display for DateTimeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for DateTimeField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| DateTimeField {
            value: t,
            date: None,
            errors: Vec::new(),
        })
    }
}

impl Deref for DateTimeField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DateTimeField {
    fn deref_mut(&mut self) -> &mut Self::Target{
        &mut self.value
    }
}

impl From<String> for DateTimeField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new(), date: None }
    }
}

impl FromStr for DateTimeField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new(), date: None })
    }
}

impl DateTimeField {
    fn suggest_date_format(date: &str) ->String{
        let mut suggested_date_fmt = "%Y-%m-%d %H:%M:%S";
        if date.contains("/") {
            if date.contains("T"){
                suggested_date_fmt = "%Y/%m/%dT%H:%M:%S";
            } else {
                suggested_date_fmt = "%Y/%m/%d %H:%M:%S";
            }
        }
        else if date.contains("-") {
            if date.contains("T"){
                suggested_date_fmt = "%Y-%m-%dT%H:%M:%S";
            }
        }
        suggested_date_fmt.into()
    }
}

impl Validation for DateTimeField {
    fn is_valid_field(&mut self, field_name: &str) -> bool {
        let date_fmt = Self::suggest_date_format(&self.value);
        match NaiveDateTime::parse_from_str(&self.value, &date_fmt) {
            Ok(date) => {
                self.date = Some(date);
                true
            }
            Err(e) => {
                if date_fmt.contains("T"){
                    let date = 
                        NaiveDateTime::parse_from_str(&self.value, &(date_fmt + "%z"));
                    if date.is_ok() {
                        self.date = Some(date.unwrap());
                        return true;
                    }
                }
                error!("Error parsing DateTimeField: {}", e);
                self.errors.push(format!("`{field_name}`: Invalid datetime format."));
                false
            }
        }
    }
}
