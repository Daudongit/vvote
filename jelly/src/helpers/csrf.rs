use rand::rngs::StdRng;
use crate::actix_web::http::Method;
use crate::guards::csrf::CsrfMiddleware;

pub trait SetRoutes{
    fn set_csrf_routes(self, routes: Vec<(Method, String)>)->Self;
}

impl SetRoutes for CsrfMiddleware<StdRng> {
    fn set_csrf_routes(self, routes: Vec<(Method, String)>)->Self{
        let mut this = self;
        for (method, route) in routes.into_iter() {
            this = this.set_cookie(method, route);
        }
        this
    }
}
