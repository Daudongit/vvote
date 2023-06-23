pub mod login;
pub mod admin;

use jelly::prelude::Context;

use crate::auth::forms::AdminLoginForm;

fn create_context(error: &str, form: &AdminLoginForm) -> Context{
    let mut context = Context::new();
    context.insert("error", error);
    context.insert("form", form);
    context  
}
