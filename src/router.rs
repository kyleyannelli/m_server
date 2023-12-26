use std::{
    collections::HashMap,
    net::TcpStream,
};

use crate::http::{
    request::{HttpRequest, HttpRequestMethod, HttpRequestFailure},
    response::HttpResponse,
};

use regex::Regex;

struct RouteHandler {
    regex: Regex,
    handler: Box<dyn Fn(HttpRequest) + Send + Sync>,
}

pub struct HttpRouter {
    // Arc is an Atomic wrapper to make the HashMap thread safe
    //  each thread will get a clone of the wrapped data to achieve this
    routes: HashMap<HttpRequestMethod, Vec<RouteHandler>>,
}

impl HttpRouter {
    pub fn new() -> HttpRouter {
        log::debug!("Router created! Added routes will be output to debug.");
        HttpRouter {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: HttpRequestMethod, path: &str, handler: F)
    where
        F: Fn(HttpRequest) + 'static + Send + Sync,
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
        self.routes
            .entry(method)
            .or_insert(Vec::<RouteHandler>::new())
            .push(route_handler);
    }

    pub fn handle_request(&self, stream: TcpStream) {
        log::debug!("Handling request!");
        let start_time = std::time::Instant::now();
        let h_req: Result<HttpRequest, HttpRequestFailure> = HttpRequest::new(stream);
        let elapsed = start_time.elapsed();
        log::debug!("Request parsing took {} microseconds", elapsed.as_micros());
        match h_req {
            Ok(mut http_req) => {
                let req_ip: String = match &http_req.peer_addr {
                    Some(addr) => addr.clone(),
                    None => "IP DNE | Check Logs!".to_owned(),
                };
                log::info!("{} {} {}", req_ip, http_req.route.method, http_req.route.path);
                let routes = &self.routes;

                if let Some(handlers) = routes.get(&http_req.route.method) {
                    for handler in handlers {
                        if handler.regex.is_match(&http_req.route.path) {
                            (handler.handler)(http_req);
                            return;
                        }
                    }
                    // respond with 404
                    http_req.respond(HttpResponse::not_found());
                } else {
                    // respond with 404
                    http_req.respond(HttpResponse::not_found());
                }
            },
            Err(mut http_fail) => {
                log::error!("Error occured from HttpRequest: \n\t{}", http_fail.fail_reason);
                http_fail.respond(HttpResponse::bad_request());
            }
        }
    }

    fn convert_path_to_regex(&self, path: &str) -> String {
        let mut regex_pattern = "^".to_string();
        for segment in path.split('/') {
            if !segment.is_empty() {
                regex_pattern.push('/');
            }
            if segment.starts_with('{') && segment.ends_with('}') {
                let param_name = &segment[1..segment.len() - 1];
                regex_pattern.push_str(&format!("(?P<{}>[^/]+)", param_name));
            } else {
                regex_pattern.push_str(segment);
            }
        }
        regex_pattern.push('$');
        regex_pattern.clone()
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
