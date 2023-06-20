
use serde_json::Value;
use actix_web::HttpRequest;
use actix_web::HttpMessage as _;
use actix_session::SessionExt as _;

use crate::accounts::User;
use crate::error::error::Error;
use crate::guards::{AuthConfig, AuthSessionName};

/// `Authentication` is kind of a request guard - it returns a Future which will resolve
/// with either the current authenticated user, or "error" out if the user has no session data
/// that'd tie them to a user profile, or if the session cache can't be read, or if the database
/// has issues, or... pick your poison I guess.
///
pub trait Authentication {
    /// Returns whether a user session exists and is valid.
    fn is_authenticated(&self, auth_config: AuthConfig) -> Result<(bool, String), Error>;

    /// Returns whether a user session is missing(absent).
    fn is_guest(&self, auth_config_name: &AuthSessionName) -> Result<bool, Error>;

    /// Sets a serializable user instance.
    fn set_user(&self, auth_config_name: &AuthSessionName, account: User) -> Result<(), Error>;

    /// Returns a User, if it can be extracted properly.
    fn user(&self, auth_config_name: &AuthSessionName) -> Result<User, Error>;
}

impl Authentication for HttpRequest {
    // #[inline(always)]
    fn is_authenticated(&self, auth_config: AuthConfig) -> Result<(bool, String), Error> {
        let auth_config_name = auth_config.name.as_ref();
        let is_authorized =
            self.get_session().get::<Value>(auth_config_name)?.is_some();
        if is_authorized {self.extensions_mut().insert(auth_config.name);}
        Ok((is_authorized, auth_config.redirect_to))
    }

    fn is_guest(&self, auth_config_name: &AuthSessionName) -> Result<bool, Error> {
        Ok(!self.get_session().get::<Value>(auth_config_name.as_ref())?.is_some())
    }

    fn set_user(&self, auth_config_name: &AuthSessionName, account: User) -> Result<(), Error> {
        self.get_session().insert(auth_config_name.as_ref(), account)?;
        Ok(())
    }

    fn user(&self, auth_config_name: &AuthSessionName) -> Result<User, Error> {
        match self.get_session().get(auth_config_name.as_ref())? {
            Some(user) => Ok(user),
            None => Ok(User::default()),
        }
    }
}
