use super::{Validation, ParseStringError, Validator::{self, Max, Min, Contain}};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::ops::Deref;
use std::fmt;


/// A generic field for validating that an input is not blank.
/// In truth, if you don't want to easily check this, you could just use a
/// `String` instead - but if you want to keep the same conventions
/// (e.g, `errors`) then feel free to use this.
#[derive(Debug, Default, Serialize)]
pub struct TextField {
    pub value: String,
    pub errors: Vec<String>,
}

impl fmt::Display for TextField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for TextField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| TextField {
            value: t,
            errors: Vec::new(),
        })
    }
}

impl Deref for TextField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for TextField {
    fn from(text: String)->Self{
        Self { value: text, errors: Vec::new() }
    }
}

impl FromStr for TextField {
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.into(), errors: Vec::new() })
    }
}

impl Validation for TextField {
    fn is_valid_field(&mut self, field: &str) -> bool {
        if self.value == "" {
            self.errors.push(format!("{field} cannot be blank."));
            return false;
        }
        true
    }

    fn is_valid_with_validators(&mut self, field_name: &str, validators: Vec<Validator>) -> bool {
        self.is_valid_field(field_name);
        for validator in validators {
            match validator {
                Min(min_value) if self.value.len() < min_value  => {
                    self.errors.push(
                        format!("{field_name} can not be less than {min_value} character.")
                    );
                },
                Max(max_value) if self.value.len() > max_value => {
                    self.errors.push(
                        format!("{field_name} can not be more than {max_value} character.")
                    );
                },
                Contain(value) if !self.value.contains(value) => {
                    self.errors.push(
                        format!("{field_name} must contain {value}.")
                    );
                },
                // other caseses pass validation
                Max(_) | Min(_) | Contain(_) => {}
            }
        }
        self.errors.len() == 0
    }
}
