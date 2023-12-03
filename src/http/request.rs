use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

use crate::{LineOrError, router::HttpRoute};

use super::response::HttpResponse;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum HttpRequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    BadRequest,
}

impl std::fmt::Display for HttpRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpRequestMethod::Get => write!(f, "GET"),
            HttpRequestMethod::Put => write!(f, "PUT"),
            HttpRequestMethod::Patch => write!(f, "PATCH"),
            HttpRequestMethod::Post => write!(f, "POST"),
            HttpRequestMethod::Delete => write!(f, "DELETE"),
            HttpRequestMethod::BadRequest => write!(f, "BAD_REQUEST"),
        }
    }
}

#[derive(Clone)]
pub struct HttpRequestParser {
    raw_req: Vec<LineOrError>,
}

impl HttpRequestParser {
    pub fn new(raw_req: Vec<LineOrError>) -> HttpRequestParser {
        HttpRequestParser {
            raw_req,
        }
    }

    pub fn method(&self) -> HttpRequestMethod {
        // attempt to get first row which should contain method & path
        match self.raw_req.first() {
            Some(method) => {
                match method {
                    LineOrError::Line(line) => Self::determine_method(line),
                    LineOrError::Error(_) => HttpRequestMethod::BadRequest,
                }
            },
            None => HttpRequestMethod::BadRequest
        }
    }

    pub fn path(&self) -> String {
        // attempt to get first row which should contain method & path
        match self.raw_req.first() {
            Some(method) => {
                match method {
                    LineOrError::Line(line) => Self::determine_path(line),
                    LineOrError::Error(_) => "/".to_string(),
                }
            },
            None => "/".to_string(),
        }
    }

    pub fn determine_path(line: &String) -> String {
        match line.split_whitespace().nth(1) {
            Some(path) => path.to_string(),
            None => "/".to_string(),
        }
    }

    fn determine_method(line: &String) -> HttpRequestMethod {
        match line {
           _ if line.as_str().starts_with("GET") => HttpRequestMethod::Get,
           _ if line.as_str().starts_with("POST") => HttpRequestMethod::Post,
           _ if line.as_str().starts_with("PUT") => HttpRequestMethod::Put,
           _ if line.as_str().starts_with("PATCH") => HttpRequestMethod::Patch,
           _ if line.as_str().starts_with("DELETE") => HttpRequestMethod::Delete,
           _ => HttpRequestMethod::BadRequest,
        }
    }
}

//#[derive(Clone)]
pub struct HttpRequest {
    #[allow(dead_code)]
    pub raw_req: Vec<LineOrError>,
    pub raw_req_string: String,
    pub tcp_stream: TcpStream,
    pub route: HttpRoute,
    pub req_parser: HttpRequestParser,
}

impl HttpRequest {
    pub fn new(mut stream: TcpStream) -> HttpRequest {
        let raw_req: Vec<LineOrError> = Self::gen_raw_req(&mut stream);
        let raw_req_string: String = Self::gen_req_str(&raw_req);
        let req_parser: HttpRequestParser = HttpRequestParser::new(raw_req.clone());

        let route: HttpRoute = HttpRoute {
            method: req_parser.method(),
            path: req_parser.path(),
        };

        HttpRequest {
            raw_req,
            raw_req_string,
            tcp_stream: stream,
            route,
            req_parser,
        }
    }

    pub fn println_req(&self) {
        let mut route_str: String = "".to_string();
        route_str.push_str(self.route.to_string().as_str());
        route_str.push_str(&self.raw_req_string);
        log::info!("{}", route_str);
    }

    pub fn respond(&mut self, http_res: HttpResponse) {
        self.tcp_stream.write_all(http_res.response.as_bytes()).unwrap();
    }

    pub fn respond_with_body(&mut self, http_res: HttpResponse, body: &str) {
        let mut res_with_body: String = http_res.response.clone();
        res_with_body.push_str(body);
        self.tcp_stream.write_all(res_with_body.as_bytes()).unwrap();
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
        let mut req_string_mut: String = "".to_owned();

        for req_line in req {
            req_string_mut.push_str("\n\t");
            req_string_mut.push_str(req_line.to_string().as_str());
        }
        req_string_mut.push_str("\n");

        let req_string: String = req_string_mut.to_string();

        return req_string;
    }
}

