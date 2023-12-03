use std::net::TcpListener;

use crate::{router::HttpRouter, http::request::HttpRequest, logger};

pub struct HttpServer {
    #[allow(dead_code)]
    bind_addr: String,
    tcp_listener: TcpListener,
}

impl HttpServer {
    /// Begins listening for http requests on the bind_addr
    ///
    /// # Arguments
    ///
    /// * `bind_addr` - String in expected format of ip:port
    pub fn new(bind_addr: &str) -> HttpServer {
        logger::MServerLogger::setup();
        let tcp_listener = Self::start_listening(bind_addr);

        HttpServer {
            bind_addr: bind_addr.to_string(),
            tcp_listener,
        }
    }

    /// Begins handling incoming connections.
    ///
    pub fn start(&self, router: HttpRouter) {
        for stream_res in self.tcp_listener.incoming() {
            match stream_res {
                Ok(result) => {
                    let http_req: HttpRequest = HttpRequest::new(result);
                    // commenting this out until router impl is done
                    // Self::handle_connection(http_req);
                    router.handle_request(&http_req);
                },
                Err(error) => match error.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        log::warn!("Waiting for network socket to be ready");
                        continue;
                    },
                    _ => {
                        let err: String = error.to_string();
                        log::error!("{}", err);
                    }
                }
            }
        }
    }

    fn start_listening(bind_addr: &str) -> TcpListener {
        match TcpListener::bind(bind_addr) {
            Ok(lis) => {
                log::info!("{} {}", "Server bound on", bind_addr);
                return lis;
            },
            Err(error) => match error.kind() {
                std::io::ErrorKind::AddrInUse => {
                    log::error!("Address {} already in use. Please make sure an instance is not already running, or no other services use the port.", bind_addr);
                    std::process::exit(1);
                },
                _ => {
                    log::error!("Error occurred while binding: {} ", error.to_string());
                    std::process::exit(1);
                }
            }
        }
    }
}
