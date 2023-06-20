use jelly::actix_web::http::Method;

/// csrf token generated routes list
pub fn csrf_routes() ->Vec<(Method, String)>{ 
    vec![
        (Method::GET, "/".into()),
        (Method::GET, "/admin/".into()),
        (Method::GET, "/auth/login/admin/".into()), // /admin alias
        (Method::GET, "/admin/nominees/".into()),
        (Method::GET, "/admin/elections/".into()),
        (Method::GET, "/admin/positions/".into()),
        (Method::GET, "/admin/slots/".into()),
        (Method::GET, "/elections/".into()),
        (Method::POST, "/elections/show/".into())
    ]
}
