# <img src="/assets/logo.png" alt="Logo" width="120" /> m_server
Super minimal HTTP server framework written in Rust.
# Getting Started
#### Below is a super basic example of creating a server and routes.
```rust
use m_server::{
  server::HttpServer,
  router::HttpRouter,
  http::{
    response::HttpResponse,
    request::HttpRequestMethod,
  }
};

// must be in the IP:PORT format!

const BIND_ADDR: &str = "127.0.0.1:7878";

fn main() {
  // creating a new HttpServer will instantly attempt to bind to the IP:PORT
  let http_server: HttpServer = HttpServer::new(self::BIND_ADDR);
  let mut router: HttpRouter = HttpRouter::new();

  // It is recommended to define the handlers in Controllers, rather than inline.
  router.add_route(HttpRequestMethod::Get, "/person".to_string(), |mut http_request| {
    let json_data = "
    {
      \"name\": \"John Doe\",
      \"age\": 22,
    }";

    http_request.respond_with_body(HttpResponse::created(), json_data);
  });
  // example of responding without body
  router.add_route(HttpRequestMethod::Delete, "/person".to_string(), |mut http_request| {
    http_request.respond(HttpResponse::ok());
  });

  // now the server can start, this is sync, hence why routes are created before start
  http_server.start(router);
}
```
