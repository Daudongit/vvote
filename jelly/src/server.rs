use std::sync::Arc;
use rand::rngs::StdRng;
use std::cell::RefCell;
use crate::guards::AuthConfig;
use actix_web::web::ServiceConfig;
// use crate::guards::{Auth, AuthConfig};
use crate::email::{Configurable, Email};
use crate::guards::csrf::CsrfMiddleware;
use crate::helpers::csrf::SetRoutes as _;
use actix_web::dev::{ResourceMap, Service}; //, ServiceRequest, ServiceFactory};
use crate::helpers::config::get_config_datas;
use crate::helpers::utils::default_service_handler;
use actix_web::{dev, middleware, web, App, HttpServer};

thread_local! {
    pub static ROUTES_KEY: RefCell<Option<ResourceMap>> = RefCell::new(None);
}

type AppFn = Box<dyn Fn(&mut ServiceConfig) + Send + Sync + 'static>;
/// This struct provides a slightly simpler way to write `main.rs` in
/// the root project, and forces more coupling to app-specific modules.
pub struct Server {
    apps: Vec<AppFn>,
    auth_config: Vec<AuthConfig>,
    csrf_routes: Vec<String>
}

impl Server {
    /// Creates a new Server struct to configure.
    pub fn new() -> Self {
        Self { apps: vec![], auth_config: vec![], csrf_routes: vec![] }
    }

    /// initialise env, logger and email config
    fn init(){
        dotenv::dotenv().ok();
        pretty_env_logger::init();
        Email::check_conf();
    }

    /// Registers a csrf_routes.
    pub fn register_csrf_routes(mut self, csrf_routes: Vec<String>) -> Self {
        self.csrf_routes = csrf_routes; self
    }

    /// Registers a auth_config.
    pub fn register_auth_config(mut self, auth_config: AuthConfig) -> Self {
        self.auth_config.push(auth_config); self
    }

    /// Registers a service.
    pub fn register_service<F>(mut self, handler: F) -> Self
    where F: Fn(&mut ServiceConfig) + Send + Sync + 'static {
        self.apps.push(Box::new(handler)); self
    }

    /// Consumes and then runs the server, with default settings that we
    /// generally want.
    pub async fn run(self) -> std::io::Result<dev::Server> {
        Self::init();
        let (
            bind, _root_domain, templates, pool
        ) = get_config_datas().await;

        let apps = Arc::new(self.apps);
        // let auth_config = Arc::new(self.auth_config);
        let csrf_routes = Arc::new(self.csrf_routes);
        // let set_routes_maps = 
        //     move |req:ServiceRequest, srv: T| {
        //     ROUTES_KEY.with(|routes| {
        //         routes.borrow_mut().replace(req.resource_map().clone());
        //     });
        //     srv.call(req)
        // };
        let server = HttpServer::new(move || {
            let csrf = 
                CsrfMiddleware::<StdRng>::new().enabled(true).set_csrf_routes(csrf_routes.to_vec());
            let mut app = App::new()
                .app_data(pool.clone())
                .app_data(templates.clone())
                .wrap(crate::helpers::session::create_session())
                .wrap(csrf)
                .wrap(middleware::Logger::default())
                // .wrap(Auth{auth_config: RefCell::new(Some(auth_config.to_vec()))})
                // .wrap_fn(set_routes_maps)
                .wrap_fn(move |req, srv| {
                    ROUTES_KEY.with(|routes| {
                        routes.borrow_mut().replace(req.resource_map().clone());
                    });
                    srv.call(req)
                })
                // Depending on your CORS needs, you may opt to change this
                // block. Up to you.
                .default_service(web::to(default_service_handler))
                .configure(crate::helpers::utils::static_handler);

            for handler in apps.iter() {
                app = app.configure(|c| handler(c));
            }
            app
        })
        .backlog(8192)
        .shutdown_timeout(0)
        .workers(4)
        .bind(&bind)?
        .run();

        Ok(server)
    }
}


// expected signature of `for<'r> fn(ServiceRequest, &'r <impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<middleware::logger::StreamLog<BoxBody>>, Error = actix_web::Error, InitError = ()> as ServiceFactory<ServiceRequest>>::Service)