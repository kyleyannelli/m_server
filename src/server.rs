use std::{net::TcpListener, sync::{Arc, RwLock}};

use tokio::runtime::Builder;

use crate::{router::HttpRouter, http::request::{HttpRequest, HttpRequestMethod}, logger};

lazy_static! {
    static ref ROUTER: Arc<RwLock<HttpRouter>> = Arc::new(RwLock::new(HttpRouter::new()));
}

pub struct HttpServer {
    #[allow(dead_code)]
    bind_addr: String,
    tcp_listener: TcpListener,
    pool_size: usize,
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
            pool_size: 12,
        }
    }

    pub fn add_route(&mut self, method: HttpRequestMethod, path: &str, handler: fn(&mut HttpRequest)) {
        match ROUTER.write() {
            Ok(mut router) => {
                router.add_route(method, path, handler);
            },
            Err(error) => {
                log::error!("Failed to get router lock! Route not added!\n\t{}", error.to_string());
            }
        };
    }

    pub fn set_pool_size(mut self, pool_size: usize) -> HttpServer {
        self.pool_size = pool_size;
        self
    }

    /// Begins handling incoming connections.
    ///
    pub fn start(&self) {
        let runtime = Builder::new_multi_thread()
            .worker_threads(self.pool_size)
            .enable_all()
            .build()
            .unwrap();


        for stream_res in self.tcp_listener.incoming() {
            log::debug!("RES");
            match stream_res {
                Ok(stream) => {
                    runtime.spawn(async move {
                        log::debug!("TOKIOOOOOOO");
                        match ROUTER.read() {
                            Ok(ok_router) => {
                                log::debug!("OK ROUTER");
                                ok_router.handle_request(stream);
                            },
                            Err(error) => {
                                log::error!("Failed to get router lock!\n\t{}", error.to_string());
                                return;
                            }
                        };
                    });
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
        runtime.shutdown_timeout(std::time::Duration::from_secs(30));
    }

    fn start_listening(bind_addr: &str) -> TcpListener {
        match TcpListener::bind(bind_addr) {
            Ok(lis) => {
                log::info!("{} {}", "Server bound on", bind_addr);
                lis
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
