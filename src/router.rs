use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use threadpool::ThreadPool;

use crate::http::{
    request::{HttpRequest, HttpRequestMethod},
    response::HttpResponse,
};

use regex::Regex;

struct RouteHandler {
    regex: Regex,
    handler: Box<dyn Fn(Result<MutexGuard<'_, HttpRequest>, MutexGuard<'_, HttpRequest>>) + Send + Sync>,
}

pub struct HttpRouter {
    // Arc is an Atomic wrapper to make the HashMap thread safe
    //  each thread will get a clone of the wrapped data to achieve this
    routes: Arc<HashMap<HttpRequestMethod, Vec<RouteHandler>>>,
    pool: ThreadPool,
}

impl HttpRouter {
    pub fn new(pool_size: usize) -> HttpRouter {
        log::debug!("Router created! Added routes will be output to debug.");
        HttpRouter {
            routes: Arc::new(HashMap::new()),
            pool: ThreadPool::new(pool_size),
        }
    }

    pub fn add_route<F>(&mut self, method: HttpRequestMethod, path: &str, handler: F)
    where
        F: Fn(Result<MutexGuard<'_, HttpRequest>, MutexGuard<'_, HttpRequest>>) + 'static + Send + Sync,
    {
        let regex_pattern = self.convert_path_to_regex(path);
        let regex = match Regex::new(&regex_pattern) {
            Ok(reg) => reg,
            Err(e) => {
                log::error!("Path not added! Issue creating regex for {}!\n\t{}", path, e);
                return;
            }
        };
        let route_handler = RouteHandler {
            regex,
            handler: Box::new(handler),
        };
        log::debug!(
            "{} {} | Regex: {}",
            method,
            path,
            route_handler.regex.to_string()
        );
        let routes = match Arc::get_mut(&mut self.routes) {
            Some(routes) => routes,
            None => {
                log::error!("Failed to create route! Unable to mutate routes object to add route.\n\t Please report this error at https://github.com/kyleyannelli/m_server");
                std::process::exit(1);
            }
        };
        routes
            .entry(method)
            .or_insert(Vec::<RouteHandler>::new())
            .push(route_handler);
    }

    pub fn handle_request(&self, request: Arc<Mutex<HttpRequest>>) {
        let req = match request.lock() {
            Ok(req) => req,
            Err(poisoned_error) => {
                log::warn!("Poisoned mutex in handle request! Continuing, but data is likely unreliable!"); 
                poisoned_error.into_inner()
            }
        };
        let req_ip: String = match &req.peer_addr {
            Some(addr) => addr.clone(),
            None => "IP DNE | Check Logs!".to_owned(),
        };
        log::info!("{} {} {}", req_ip, req.route.method, req.route.path);
        drop(req);
        // here we have to clone the routes to access it inside of the thread pool, otherwise it's
        //  an illegal move
        let routes = self.routes.clone();

        self.pool.execute(move || {
            // request the object, this will await anything using it
            let mut req = match request.lock() {
                Ok(req) => req,
                Err(poisoned_error) => {
                    log::warn!("Poisoned mutex in handle request! Continuing, but data is likely unreliable!"); 
                    poisoned_error.into_inner()
                }
            };            
            if let Some(handlers) = routes.get(&req.route.method) {
                for handler in handlers {
                    if handler.regex.is_match(&req.route.path) {
                        // we no longer want the req at this point, as we pass to the handler its
                        // out of scope
                        drop(req);
                        (handler.handler)(match request.lock() {
                            Ok(req) => Ok(req),
                            Err(poisoned_req) => {
                                Err(poisoned_req.into_inner())
                            },
                        });
                        return;
                    }
                }
                // respond with 404
                req.respond(HttpResponse::not_found());
            } else {
                // respond with 404
                req.respond(HttpResponse::not_found());
            }
            // probably not needed, but good habit to drop to avoid potential deadlock
            drop(req);
        });
    }

    fn convert_path_to_regex(&self, path: &str) -> String {
        let mut regex_pattern = "^".to_string();
        for segment in path.split('/') {
            if !segment.is_empty() {
                regex_pattern.push_str("/");
            }
            if segment.starts_with("{") && segment.ends_with("}") {
                let param_name = &segment[1..segment.len() - 1];
                regex_pattern.push_str(&format!("(?P<{}>[^/]+)", param_name));
            } else {
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
