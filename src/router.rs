use std::{
    collections::HashMap,
    sync::Arc,
};

use threadpool::ThreadPool;

use crate::http::{
    request::{HttpRequest, HttpRequestMethod},
    response::HttpResponse
};

pub struct HttpRouter {
    // Arc is an Atomic wrapper to make the HashMap thread safe
    //  each thread will get a clone of the wrapped data to achieve this
    routes: Arc<HashMap<(HttpRequestMethod, String), Box<dyn Fn(HttpRequest) + Send + Sync>>>,
    pool: ThreadPool,
}

impl HttpRouter {
    pub fn new(pool_size: usize) -> HttpRouter {
        HttpRouter {
            routes: Arc::new(HashMap::new()),
            pool: ThreadPool::new(pool_size),
        }
    }

    pub fn add_route<F>(&mut self, method: HttpRequestMethod, path: &str, handler: F)
    where
        F: Fn(HttpRequest) + 'static + Send + Sync,
    {
        let routes = match Arc::get_mut(&mut self.routes) {
            Some(routes) => routes,
            None => {
                log::error!("Failed to create route! Unable to mutate routes object to add route.");
                std::process::exit(1);
            }
        };
        routes.insert((method, path.to_owned()), Box::new(handler));
    }

    pub fn handle_request(&self, mut request: HttpRequest) {
        let route_key: (HttpRequestMethod, String) = (request.route.method.clone() , request.route.path.clone());
        // here we have to clone the routes to access it inside of the thread pool, otherwise it's
        //  an illegal move
        let routes = self.routes.clone();

        self.pool.execute(move || {
            if let Some(handler) = routes.get(&route_key) {
                request.println_req();
                handler(request);
            }
            else {
                request.println_req();
                request.respond(HttpResponse::not_found());
            }
        });
    }
}

pub struct HttpRoute {
    pub method: HttpRequestMethod,
    pub path: String,
}

impl std::fmt::Display for HttpRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method: {} Path: {}", self.method, self.path)
    }
}

