//! Implements a set of input types that can be used for Form handling. Mostly modeled after
//! Django's Form class.
//!
//! Ex:
//!
//! ```rust
//! use serde::Deserialize;
//! use jelly::forms::{EmailField, PasswordField, Validation};
//!
//! #[derive(Debug, Default, Deserialize)]
//! pub struct MyForm {
//!     pub email: EmailField,
//!     pub password: PasswordField
//! }
//!
//! impl Validation for MyForm {
//!     fn is_valid(&mut self) -> bool {
//!         self.email.is_valid() && self.password.is_valid()
//!     }
//! }
//! ```

use std::fmt;

mod booly;
pub use booly::BoolField;

mod date;
pub use date::DateField;

mod email;
pub use email::EmailField;

mod password;
pub use password::PasswordField;

mod slug;
pub use slug::SlugField;

mod text;
pub use text::TextField;

mod number;
pub use number::NumberField;

mod date_time;
pub use date_time::DateTimeField;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseStringError;

impl fmt::Display for ParseStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not supported for field type".fmt(f)
    }
}

/// List of possible validators
pub enum Validator<'a, T = usize> {
    Min(T),
    Max(T),
    Contain(&'a str)
}

/// A trait that Forms can implement for validation. Each field type implements this trait, so you
/// can simply write your validation method as `field.is_valid()`.
pub trait Validation<I = usize> {
    /// Checks if the data held is valid. Should return a bool value.
    fn is_valid(&mut self) -> bool {
        false
    }

    /// Checks if the data held is valid. Should return a bool value.
    /// specify field name
    fn is_valid_field(&mut self,  _: &str) -> bool {
        false
    }

    /// Checks if the data held is valid with validators. Should return a bool value.
    fn is_valid_with_validators(&mut self, field_name: &str, _: Vec<Validator<I>>) -> bool
    {
        self.is_valid_field(field_name)
    }

}
