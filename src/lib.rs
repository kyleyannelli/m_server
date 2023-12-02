use std::{
    net::{TcpStream, TcpListener},
    io::{prelude::*, BufReader, self}, thread::{JoinHandle, self}
};

pub struct HttpServer {
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
        let tcp_listener = Self::start_listening(bind_addr);

        HttpServer {
            bind_addr: bind_addr.to_string(),
            tcp_listener,
        }
    }

    /// Begins handling incoming connections.
    ///
    pub fn start(&self) {
        for stream_res in self.tcp_listener.incoming() {
            match stream_res {
                Ok(result) => {
                    let http_req: HttpRequest = HttpRequest::new(result);
                    Self::handle_connection(http_req);
                },
                Err(error) => match error.kind() {
                    io::ErrorKind::WouldBlock => {
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
                io::ErrorKind::AddrInUse => {
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



    fn handle_connection(mut http_req: HttpRequest) {
        http_req.println_req();

        if http_req.route.method == "GET" && http_req.route.path == "/" {
            http_req.respond(HttpResponse::accepted());
        }
        else {
            http_req.respond(HttpResponse::not_found());
        }
    }

}

pub struct HttpRoute {
    method: String,
    path: String,
}

pub struct HttpRequest {
    raw_req: Vec<LineOrError>,
    raw_req_string: String,
    tcp_stream: TcpStream,
    route: HttpRoute,
}

impl HttpRequest {
    pub fn new(mut stream: TcpStream) -> HttpRequest {
        let raw_req: Vec<LineOrError> = Self::gen_raw_req(&mut stream);
        let raw_req_string: String = Self::gen_req_str(&raw_req);
        let route: HttpRoute = HttpRoute {
            method: "GET".to_string(),
            path: "/".to_string(),
        };
        HttpRequest {
            raw_req,
            raw_req_string,
            tcp_stream: stream,
            route,
        }
    }

    pub fn println_req(&self) {
        println!("{}", self.raw_req_string);
    }

    pub fn respond(&mut self, http_res: HttpResponse) {
        self.tcp_stream.write_all(http_res.response.as_bytes()).unwrap();
    }

    fn gen_raw_req(mut stream: &TcpStream) -> Vec<LineOrError> {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<LineOrError> = buf_reader
            .lines()
            .map(|result| match result {
                Ok(res) => LineOrError::Line(res),
                Err(error) => {
                    log::error!("Error reading line:\n\t{}", error);
                    LineOrError::Error(error.to_string())
                },
            })
        .take_while(|line| match line {
            LineOrError::Line(line) => !line.is_empty(),
            LineOrError::Error(_) => true,
        })
        .collect();
        return http_request;
    }

    fn gen_req_str(req: &Vec<LineOrError>) -> String {
        let mut req_string_mut: String = "\n\t".to_owned();

        for req_line in req {
            req_string_mut.push_str("\n\t");
            req_string_mut.push_str(req_line.to_string().as_str());
        }
        req_string_mut.push_str("\n");

        let req_string: String = req_string_mut.to_string();

        return req_string;
    }
}

pub struct HttpResponse {
    response: String,
}

impl HttpResponse {
    const HTTP_VER: &'static str = "HTTP/1.1";
    const HTTP_PAD: &'static str = "\r\n\r\n";
    const RESPONSE_OK: &'static str = "200 OK";
    const RESPONSE_NOT_FOUND: &'static str = "404 Not Found";
    const RESPONSE_ERROR: &'static str = "500 Internal Server Error";
    const RESPONSE_CREATED: &'static str = "201 Created";
    const RESPONSE_ACCEPTED: &'static str = "Accepted";

    pub fn new(response: String) -> HttpResponse {
        HttpResponse {
            response,
        }
    }

    pub fn ok() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_OK);
        return HttpResponse::new(response_str);
    }

    pub fn not_found() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_NOT_FOUND);
        return HttpResponse::new(response_str);
    }

    pub fn created() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_CREATED);
        return HttpResponse::new(response_str);
    }

    pub fn accepted() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_ACCEPTED);
        return HttpResponse::new(response_str);
    }

    pub fn error() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_ERROR);
        return HttpResponse::new(response_str);
    }

    fn status_message(status_code: &str) -> String {
        return format!("{} {}{}", Self::HTTP_VER, status_code, Self::HTTP_PAD);
    }
}

pub enum LineOrError {
    Line(String),
    Error(String),
}

impl std::fmt::Debug for LineOrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LineOrError::Line(line) => write!(f, "{}", line),
            LineOrError::Error(err) => write!(f, "{}", err),
        }
    }
}

impl std::fmt::Display for LineOrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

