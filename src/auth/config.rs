use std::env;
use jelly::guards::{AuthConfig, AuthSessionName};

/// set auth configuration
pub fn auth_config() ->AuthConfig{
    let session_name = 
        env::var("VOTER_SESSION_NAME").unwrap_or("voter_session".into());
    AuthConfig{
        name:AuthSessionName(session_name), redirect_to:"/".into()
    }
}

/// set admin auth configuration
pub fn admin_auth_config() ->AuthConfig{
    let admin_session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or("admin_session".into());
    AuthConfig{
        name:AuthSessionName(admin_session_name), 
        redirect_to:"/auth/login/admin/".into()
    }
}