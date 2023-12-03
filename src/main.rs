use m_server::{
    server::HttpServer,
    router::HttpRouter,
    http::{
        response::HttpResponse,
        request::HttpRequestMethod
    }
};

/// Represents the binding address for the server.
///
/// This address is used when the server is set up to listen for incoming connections.
/// The format is `IP:PORT`.
const BIND_ADDR: &str = "127.0.0.1:7878";

fn main() {
    // setup log4rs
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(i_file) => i_file,
        Err(error) => {
            println!("Logger failed to initalize!");
            println!("Error occurred while attempting to utilize init file. Make sure it's in the root directory!: \n\t{}", error.to_string());
            std::process::exit(1);
        }
    };

    let http_server: HttpServer = HttpServer::new(self::BIND_ADDR);
    let mut router: HttpRouter = HttpRouter::new();

    router.add_route(HttpRequestMethod::Get, "/dsaj".to_string(), |mut req| {
        req.println_req();
        req.respond(HttpResponse::ok());
    });

    http_server.start(router);
    // http_server.start();
}

