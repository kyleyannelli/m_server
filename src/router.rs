use std::{
    collections::HashMap,
    sync::Arc,
};

use threadpool::ThreadPool;

use crate::http::{
    request::{HttpRequest, HttpRequestMethod},
    response::HttpResponse
};

use regex::Regex;

struct RouteHandler {
    regex: Regex,
    handler: Box<dyn Fn(&HttpRequest) + Send + Sync>,
}

pub struct HttpRouter {
    // Arc is an Atomic wrapper to make the HashMap thread safe
    //  each thread will get a clone of the wrapped data to achieve this
    routes: Arc<HashMap<HttpRequestMethod, Vec<RouteHandler>>>,
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
        F: Fn(&HttpRequest) + 'static + Send + Sync,
    {
        let regex_pattern = self.convert_path_to_regex(path);
        let regex = Regex::new(&regex_pattern).unwrap();
        let route_handler = RouteHandler {
            regex,
            handler: Box::new(handler),
        };
        let routes = match Arc::get_mut(&mut self.routes) {
            Some(routes) => routes,
            None => {
                log::error!("Failed to create route! Unable to mutate routes object to add route.\n\t Please report this error at https://github.com/kyleyannelli/m_server");
                std::process::exit(1);
            }
        };
        routes.entry(method).or_insert(Vec::<RouteHandler>::new()).push(route_handler);
    }

    pub fn handle_request(&self, mut request: &HttpRequest) {
        // here we have to clone the routes to access it inside of the thread pool, otherwise it's
        //  an illegal move
        let routes = self.routes.clone();
        let route_path = request.route.path.clone();

        self.pool.execute(move || {
            if let Some(handlers) = routes.get(&request.route.method) {
                for handler in handlers {
                    if handler.regex.is_match(&route_path) {
                        (handler.handler)(request);
                    }
                }
            }
        });
    }

    fn convert_path_to_regex(&self, path: &str) -> String {
        let mut regex_pattern = "^".to_string();
        for segment in path.split('/') {
            regex_pattern.push_str("/");
            if segment.starts_with("{") && segment.ends_with("}") {
                let param_name = &segment[1..segment.len()-1];
                regex_pattern.push_str(&format!("(?P<{}>[^/]+)", param_name));
            }
            else {
                regex_pattern.push_str(segment);
            }
        }
        regex_pattern.push_str("$");
        return regex_pattern.clone();
    }
}

#[derive(Clone)]
pub struct HttpRoute {
    pub method: HttpRequestMethod,
    pub path: String,
}

impl std::fmt::Display for HttpRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method: {} Path: {}", self.method, self.path)
    }
}

