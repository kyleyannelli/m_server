# <img src="/assets/logo.png" alt="Logo" width="120" /> m_server
Super minimal HTTP server framework for delivering JSON data; written in Rust.
# Getting Started
#### Below is a super basic example of creating a server and routes.
```rust
use m_server::{
    server::HttpServer,
    http::{
        response::HttpResponse,
        request::{HttpRequestMethod, HttpRequest},
    }
};

// must be in the IP:PORT format!
const BIND_ADDR: &str = "127.0.0.1:7878";
// thread pool size for route handling (default is 12)
const POOL_SIZE: usize = 30;

fn get_person(request: &mut HttpRequest) {
    let json_data = "
    {
        \"name\": \"John Doe\",
        \"age\": 22
    }";
    request.respond_with_body(&HttpResponse::ok(), json_data)
}

fn main() {
    // creating a new HttpServer will instantly attempt to bind to the IP:PORT
    let mut http_server: HttpServer = HttpServer::new(self::BIND_ADDR).set_pool_size(POOL_SIZE);
    http_server.add_route(HttpRequestMethod::Get, "/fort", get_person);
    // It is recommended to define the handlers in Controllers, rather than inline.
    http_server.add_route(HttpRequestMethod::Get, "/person", |http_request| {
        let json_data = "
        {
            \"name\": \"John Doe\",
            \"age\": 22,
        }";

        match &http_request.body.body_params {
            Some(params) => {
                let wanted_param = "required param";
                let msg_opt = params.get(wanted_param);
                let msg;
                match msg_opt {
                    Some(message) => msg = message.clone(),
                    None => {
                        http_request.respond_with_body(&HttpResponse::bad_request(), "Missing parameter!");
                        return;
                    }
                }
                http_request.respond_with_body(&HttpResponse::created(), &msg);
            },
            None => http_request.respond_with_body(&HttpResponse::created(), json_data)
        }
    });
    // example of responding without body
    http_server.add_route(HttpRequestMethod::Delete, "/person/{person_id}/settings", |http_request| {
        http_request.respond(HttpResponse::ok());
    });

    // example of responding without body
    http_server.add_route(HttpRequestMethod::Post, "/person", |http_request| {
        http_request.respond(HttpResponse::ok());
    });

    // now the server can start, this is sync, hence why routes are created before start
    http_server.start();
}
```

