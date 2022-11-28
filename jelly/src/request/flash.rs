use serde::Serialize;
use serde_json::Value;
use actix_web::HttpRequest;
use actix_session::SessionExt as _;

use crate::error::error::Error;
use crate::helpers::templates::FlashMessage;

/// `FlashMessages` implements a one-time-message (hence "Flash") that is useful
/// for old-school HTML flows that need to display messages in a standardized way
/// across pages.
///
/// This could potentially do less serialization, but it's fine for now.
/// TODO: Look at whether this can be done with just &str rather than String.
pub trait FlashMessages {
    /// Adds a flash message to the stack.
    fn flash(&self, title: &str, message: &str) -> Result<(), Error>;

    /// Internally used; loads flash messages for template use and removes the existing
    /// stack.
    fn get_flash_messages(&self) -> Result<Vec<FlashMessage>, Error>;

    fn flash_form<S: Serialize>(&self, form_data: S) -> Result<(), Error>;

    fn get_flash_form(&self) -> Result<Option<Value>, Error>;
}

impl FlashMessages for HttpRequest {
    fn flash(&self, title: &str, message: &str) -> Result<(), Error> {
        let session = self.get_session();
        let mut messages: Vec<FlashMessage> = match session.get("flsh")? {
            Some(messages) => messages,
            None => Vec::new(),
        };
        messages.push(FlashMessage{title: title.to_string(), message: message.to_string()});
        session.insert("flsh", messages)?;
        Ok(())
    }

    fn get_flash_messages(&self) -> Result<Vec<FlashMessage>, Error> {
        let session = self.get_session();
        let messages = match session.get("flsh")? {
            Some(messages) => messages,
            None => Vec::new(),
        };
        session.remove("flsh");
        Ok(messages)
    }

    fn flash_form<S: Serialize>(&self, form_data: S) -> Result<(), Error> {
        let session = self.get_session();
        session.insert("flsh_form", form_data)?;
        Ok(())
    }

    fn get_flash_form(&self) -> Result<Option<Value>, Error> {
        let session = self.get_session();
        let flash_form = session.get("flsh_form")?;
        session.remove("flsh_form");
        Ok(flash_form)
    }
}
