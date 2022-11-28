use jelly::Server;
use jelly::actix_web::http::Method;
use jelly::guards::{AuthConfig, AuthSessionName};

pub const AUTH_ROUTES: [(Method, &str); 3] = [
    (Method::GET, "/elections/"),
    (Method::GET, "/vote/{election}/"),
    (Method::POST, "/vote/{election}/")
];

pub const ADMIN_AUTH_ROUTES: [(Method, &str); 22] = [
    (Method::GET,   "/admin/dashboard/"),
    (Method::GET,   "/admin/results/"),
    (Method::GET,   "/admin/results/{election}/"),
    (Method::POST,  "/admin/suspend/{election}/"),
    (Method::GET,   "/admin/export/vote/{election}/{position}/"),
    (Method::GET,   "/admin/export/{election}/{slot}/"),
    (Method::GET,   "/admin/nominees/"),
    (Method::POST,  "/admin/nominees/store/"),
    (Method::POST,  "/admin/nominees/{id}/"),
    (Method::POST,  "/admin/nominees/{id}/delete/"),
    (Method::GET,   "/admin/elections/"),
    (Method::POST,  "/admin/elections/store/"),
    (Method::POST,  "/admin/elections/{id}/"),
    (Method::POST,  "/admin/elections/{id}/delete/"),
    (Method::GET,   "/admin/positions/"),
    (Method::POST,  "/admin/positions/store/"),
    (Method::POST,  "/admin/positions/{id}/"),
    (Method::POST,  "/admin/positions/{id}/delete/"),
    (Method::GET,   "/admin/slots/"),
    (Method::POST,  "/admin/slots/store/"),
    (Method::POST,  "/admin/slots/{id}/"),
    (Method::POST,  "/admin/slots/{id}/delete/")
];

// pub trait SetAuthRoutes{
//     fn set_auth_guard(self, name:&str, redirect_to:&str, routes: Box<[(Method, &str)]>)->Self;
// }

// impl SetAuthRoutes for Server {
//     fn set_auth_guard(self, name:&str, redirect_to:&str, routes: Box<[(Method, &str)]>)->Self{
//         let mut this = self;
//         this = this.register_auth_config(
//             AuthConfig{
//                 name: AuthSessionName(name.into()),
//                 redirect_to: redirect_to.into(),
//                 secure_routes: routes.into_iter()
//                     .map(|p|(p.0.clone(), p.1.to_string())).collect()
//             }
//         );
//         this
//     }
// }



// struct RoutePath(pub HashSet<(Method, String)>);
// impl From<Box<[(Method, &str)]>> for RoutePath{
//     fn from(route_paths: Box<[(Method, &str)]>) -> Self {
//         let mut routes = HashSet::new();
//         for route in route_paths.into_iter() {
//             routes.insert((route.0.clone(), route.1.into()));
//         }
//         RoutePath(routes)  
//     }
// }
// // struct RoutePath(pub HashSet<(Method, String)>);
// // impl From<[(Method, &str); 13]> for RoutePath{
// //     fn from(route_paths: [(Method, &str); 13]) -> Self {
// //         let mut admin_routes = HashSet::new();
// //         for route in route_paths {
// //             admin_routes.insert((route.0, route.1.into()));
// //         }
// //         RoutePath(admin_routes)  
// //     }
// // }

// // fn foo<T, const N>(a:[T, N]){

// // }
// pub trait SetAuthRoutes{
//     fn set_auth_routes(self, routes: Box<[(Method, &str)]>)->Self;
//     // fn set_admin_secure_routes(routes: &[(Method, String)])->HashSet<(Method, String)>;
// }

// impl SetAuthRoutes for Server {
//     fn set_auth_routes(self, routes: Box<[(Method, &str)]>)->Self{
//         let mut this = self;
//         this = this.register_auth_config(
//             AuthConfig{
//                 name: AuthSessionName("adminsku".into()),
//                 redirect_to: "/".into(),
//                 secure_routes: routes.into_iter()
//                     .map(|p|(p.0.clone(), p.1.to_string())).collect()
//             }
//         );
//         this
//     }

//     // fn set_auth_routes(self, routes: Box<[(Method, &str)]>)->Self{
//     //     let mut this = self;
//     //     let route_path:RoutePath = routes.into();
//     //     // let route_path:RoutePath = Box::new(routes).into();
//     //     this = this.register_auth_config(
//     //         AuthConfig{
//     //             name: AuthSessionName("adminsku".into()),
//     //             redirect_to: "/".into(),
//     //             secure_routes: route_path.0
//     //         }
//     //     );
//     //     this
//     // }

//     // fn set_auth_routes(self, routes: [(Method, &str); 13])->Self{
//     //     let mut this = self;
//     //     let route_path:RoutePath = routes.into();
//     //     this = this.register_auth_config(
//     //         AuthConfig{
//     //             name: AuthSessionName("adminsku".into()),
//     //             redirect_to: "/".into(),
//     //             secure_routes: route_path.0
//     //             // secure_routes: routes.iter().map(|p|(p.0, String::from(p.1))).collect()
//     //             // secure_routes: HashSet::from(routes)
//     //         }
//     //     );
//     //     this
//     // }

//     // fn set_admin_secure_routes(routes: &[(Method, String)])->HashSet<(Method, String)>{
//     //     let mut admin_routes = HashSet::new();
//     //     for &route in routes {
//     //         admin_routes.insert(route);
//     //     }
//     //     admin_routes
//     // }
// }