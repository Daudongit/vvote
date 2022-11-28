use crate::guards::csrf::CsrfMiddleware;
use crate::actix_web::http::Method;
use rand::rngs::StdRng;

pub trait SetRoutes{
    fn set_csrf_routes(self, routes: Vec<String>)->Self;
}

impl SetRoutes for CsrfMiddleware<StdRng> {
    fn set_csrf_routes(self, routes: Vec<String>)->Self{
        let mut this = self;
        for route in routes.into_iter() {
            this = this.set_cookie(Method::GET, route);
        }
        this
    }
}