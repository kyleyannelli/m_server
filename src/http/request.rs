use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

use crate::{LineOrError, router::HttpRoute};

use super::response::HttpResponse;

enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl std::fmt::Display for HttpRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpRequestMethod::GET => write!(f, "GET"),
            HttpRequestMethod::PUT => write!(f, "PUT"),
            HttpRequestMethod::PATCH => write!(f, "PATCH"),
            HttpRequestMethod::POST => write!(f, "POST"),
            HttpRequestMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

pub struct HttpRequest {
    #[allow(dead_code)]
    pub raw_req: Vec<LineOrError>,
    pub raw_req_string: String,
    pub tcp_stream: TcpStream,
    pub route: HttpRoute,
    pub method: HttpRequestMethod,
}

impl HttpRequest {
    pub fn new(mut stream: TcpStream) -> HttpRequest {
        let raw_req: Vec<LineOrError> = Self::gen_raw_req(&mut stream);
        let raw_req_string: String = Self::gen_req_str(&raw_req);
        let route: HttpRoute = HttpRoute {
            method: "GET".to_string(),
            path: "/dsaj".to_string(),
        };
        HttpRequest {
            raw_req,
            raw_req_string,
            tcp_stream: stream,
            route,
            method: HttpRequestMethod::GET,
        }
    }

    pub fn println_req(&self) {
        println!("{}", self.raw_req_string);
        println!("METHOD IS {}", self.method);
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

