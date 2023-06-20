mod number_type;

use std::ops::Deref;
use std::str::FromStr;
use std::cmp::PartialOrd;
use std::fmt::{self, Display, Formatter};

use number_type::NumberType;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

use super::{Validation, ParseStringError, Validator::{self, Max, Min, Contain}};

#[derive(Debug, Default, Serialize)]
pub struct NumberField<I = usize> {
    pub value: I,
    pub errors: Vec<String>
}

impl<I> Display for NumberField<I> where I:Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de, I> Deserialize<'de> for NumberField<I> where I: DeserializeOwned + 'static{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|t| NumberField {
            value: t,
            errors: Vec::new(),
        })
    }
}

impl<I> Deref for NumberField<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<I> From<String> for NumberField<I> where I: FromStr + Default + NumberType{
    fn from(text: String)->Self{
        Self { value: text.parse().unwrap_or_default(), errors: Vec::new() }
    }
}

impl<I> FromStr for NumberField<I> where I: FromStr + Default + NumberType{
    type Err = ParseStringError;

    fn from_str(text: &str)->Result<Self, Self::Err>{
        Ok(Self { value: text.parse().unwrap_or_default(), errors: Vec::new() })
    }
}

impl<I> Validation<I> for NumberField<I> where I: PartialOrd + Default + NumberType + Display{
    fn is_valid_field(&mut self, _: &str) -> bool {
        true // NumberType trait already verify that its number
    }

    fn is_valid_with_validators(&mut self, field_name: &str, validators: Vec<Validator<I>>) -> bool {
        self.is_valid_field(field_name);
        for validator in validators {
            match validator {
                Min(min_value) if self.value < min_value  => {
                    self.errors.push(
                        format!("{field_name} can not be less than {min_value}.")
                    );
                }
                Max(max_value) if self.value > max_value => {
                    self.errors.push(
                        format!("{field_name} can not be more than {max_value}.")
                    );
                }
                // other caseses pass validation
                Max(_) | Min(_) | Contain(_) => {}
            }
        }
        self.errors.len() == 0
    }
}
