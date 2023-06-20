pub mod upload;
pub mod result;
pub mod validation;

use std::iter::Iterator;
use std::fmt::{Write, Display};

trait JoinValue{
    fn join(&mut self, sep: &str) -> String;
}

impl<T> JoinValue for T 
    where T: Iterator, T::Item: Display
{
    fn join(&mut self, sep: &str) -> String {
        match self.next() {
            None => String::new(),
            Some(first_elt) => {
                // estimate lower bound of capacity needed
                let (lower, _) = self.size_hint();
                let mut result = String::with_capacity(sep.len() * lower);
                write!(&mut result, "{}", first_elt).unwrap();
                self.for_each(|elt| {
                    result.push_str(sep);
                    write!(&mut result, "{}", elt).unwrap();
                });
                result
            }
        }
    }
}
