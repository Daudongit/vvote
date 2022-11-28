use std::env;
use jelly::guards::{AuthConfig, AuthSessionName};

/// csrf token generated routes list
pub fn csrf_routes() ->Vec<String>{
    vec![
        "/".into(),
        "/admin/".into(),
        "/login/admin/".into(),
        "/admin/nominees/".into(),
        "/admin/elections/".into(),
        "/admin/positions/".into(),
        "/admin/slots/".into()
    ]
}


/// set admin auth configuration
pub fn get_auth_config() ->AuthConfig{
    let admin_session_name = 
        env::var("ADMIN_SESSION_NAME").unwrap_or_else(|_| "".into());
    AuthConfig{
        name:AuthSessionName(admin_session_name), redirect_to:"/login/admin/".into()
    }
}