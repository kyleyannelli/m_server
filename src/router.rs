use std::collections::HashMap;

use crate::http::{request::HttpRequest, response::HttpResponse};

pub struct HttpRouter {
    routes: HashMap<(String, String), Box<dyn Fn(HttpRequest) + Send + Sync>>,
}

impl HttpRouter {
    pub fn new() -> HttpRouter {
        HttpRouter {
            routes: HashMap::new()
        }
    }

    pub fn add_route<F>(&mut self, method: String, path: String, handler: F)
    where
        F: Fn(HttpRequest) + 'static + Send + Sync,
    {
        self.routes.insert((method, path), Box::new(handler));
    }

    pub fn handle_request(&self, mut request: HttpRequest) {
        let route_key: (String, String) = (request.route.method.clone(), request.route.path.clone());
        if let Some(handler) = self.routes.get(&route_key) {
            handler(request);
        }
        else {
            request.println_req();
            request.respond(HttpResponse::not_found());
        }
    }
}

pub struct HttpRoute {
    pub method: String,
    pub path: String,
}