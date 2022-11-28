
use crate::guards::{AuthConfig, AuthSessionName};
use actix_session::UserSession as _;
use actix_web::HttpRequest;
use crate::accounts::User;
use crate::error::Error;
use serde_json::Value;

/// `Authentication` is kind of a request guard - it returns a Future which will resolve
/// with either the current authenticated user, or "error" out if the user has no session data
/// that'd tie them to a user profile, or if the session cache can't be read, or if the database
/// has issues, or... pick your poison I guess.
///
pub trait Authentication {
    /// Returns whether a user session exists and is valid.
    fn is_authenticated(&self, auth_config: Vec<AuthConfig>) -> Result<(bool, String), Error>;

    /// Returns whether a user session is missing(absent).
    fn is_guest(&self, auth_config_name: AuthSessionName) -> Result<bool, Error>;

    /// Sets a serializable user instance.
    fn set_user(&self, auth_config_name: AuthSessionName, account: User) -> Result<(), Error>;

    /// Returns a User, if it can be extracted properly.
    fn user(&self, auth_config_name: AuthSessionName) -> Result<User, Error>;
}

impl Authentication for HttpRequest {
    #[inline(always)]
    fn is_authenticated(&self, auth_config: Vec<AuthConfig>) -> Result<(bool, String), Error> {
        let mut current_config = None;
        for config in auth_config {
            let is_match_path = self.match_pattern().map_or_else(||{
                    config.secure_routes.contains(&(
                        self.method().clone(), self.path().replace("//","/")
                    ))
                },
                |p|config.secure_routes.contains(&(self.method().clone(), p.replace("//","/")))
            );
            if is_match_path { current_config = Some(config); break; }
        }
        match current_config {
            None => Ok((true, "".into())),
            Some(config) => {
                let auth_config_name = config.name.as_ref();
                let is_authorized =
                    dbg!(self.get_session().get::<Value>(auth_config_name))?.is_some();
                if is_authorized {self.extensions_mut().insert(config.name)};
                Ok((is_authorized, config.redirect_to))
            }
        }
    }

    fn is_guest(&self, auth_config_name: AuthSessionName) -> Result<bool, Error> {
        Ok(!self.get_session().get::<Value>(auth_config_name.as_ref())?.is_some())
    }

    fn set_user(&self, auth_config_name: AuthSessionName, account: User) -> Result<(), Error> {
        self.get_session().set(auth_config_name.as_ref(), account)?;
        Ok(())
    }

    fn user(&self, auth_config_name: AuthSessionName) -> Result<User, Error> {
        match self.get_session().get(auth_config_name.as_ref())? {
            Some(user) => Ok(user),
            None => Ok(User::default()),
        }
    }
}
