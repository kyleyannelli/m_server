use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::{router::HttpRoute, LineOrError};

use super::{response::HttpResponse, shared::HttpHeaderBody};

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

pub struct HttpRequestParser;

impl HttpRequestParser {
    pub fn method(raw: &Vec<LineOrError>) -> HttpRequestMethod {
        // attempt to get first row which should contain method & path
        match raw.first() {
            Some(method) => match method {
                LineOrError::Line(line) => Self::determine_method(line),
                LineOrError::Error(_) => HttpRequestMethod::BadRequest,
            },
            None => HttpRequestMethod::BadRequest,
        }
    }

    pub fn path(raw: &Vec<LineOrError>) -> String {
        // attempt to get first row which should contain method & path
        match raw.first() {
            Some(method) => match method {
                LineOrError::Line(line) => Self::determine_path(line),
                LineOrError::Error(_) => "/".to_string(),
            },
            None => "/".to_string(),
        }
    }

    pub fn determine_path(line: &str) -> String {
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
            _ => {
                log::debug!("Unidentified HTTP Request \"{}\"", line);
                HttpRequestMethod::BadRequest
            }
        }
    }
}

pub struct HttpRequestFailure {
    pub tcp_stream: TcpStream,
    pub fail_reason: String,
}

impl HttpRequestFailure {
    pub fn respond(&mut self, http_res: HttpResponse) {
        match self.tcp_stream.write_all(http_res.response.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to write to TcpStream in respond!\n\t{}", e);
            }
        }
    }

    pub fn respond_with_body(&mut self, http_res: &HttpResponse, body: &str) {
        let mut res_with_body: String = String::new();
        res_with_body.push_str(&http_res.response);
        res_with_body.push_str(body);
        match self.tcp_stream.write_all(res_with_body.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to write to TcpStream in respond with body!\n\t{}", e);
            }
        };
    }
}

//#[derive(Clone)]
pub struct HttpRequest {
    #[allow(dead_code)]
    pub tcp_stream: TcpStream,
    pub route: HttpRoute,
    pub peer_addr: Option<String>,
    pub raw_req_string: String,
    pub body: HttpHeaderBody,
}

impl HttpRequest {
    pub fn new(stream: TcpStream) -> Result<HttpRequest, HttpRequestFailure> {
        let h_body = Self::gen_raw_req(stream);
        match h_body {
            Ok((header_body, stream)) => {
                let raw_req_string = Self::gen_req_str(&header_body.lines); 
                log::debug!("{}", raw_req_string);

                let route: HttpRoute = HttpRoute {
                    method: HttpRequestParser::method(&header_body.lines),
                    path: HttpRequestParser::path(&header_body.lines),
                };

                let peer_addr: Option<String> = match &stream.peer_addr() {
                    Ok(addr) => Some(addr.ip().to_string()),
                    Err(e) => {
                        log::error!("Socket Address for peer failed! \n\t{}", e);
                        None
                    }
                };

                Ok(HttpRequest {
                    tcp_stream: stream,
                    route,
                    peer_addr,
                    raw_req_string,
                    body: header_body,
                })
            },
            Err((reason_str, stream)) => {
                Err(
                    HttpRequestFailure {
                        tcp_stream: stream,
                        fail_reason: reason_str,
                    }
                   )
            }
        }
    }

    pub fn println_req(&self) {
        let mut route_str: String = "".to_string();
        route_str.push_str(self.route.to_string().as_str());
        route_str.push_str(&self.raw_req_string);
        log::info!("{}", route_str);
    }

    pub fn respond(&mut self, http_res: HttpResponse) {
        match self.tcp_stream.write_all(http_res.response.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to write to TcpStream in respond!\n\t{}", e);
            }
        }
    }

    pub fn respond_with_body(&mut self, http_res: &HttpResponse, body: &str) {
        let mut res_with_body: String = String::new();
        res_with_body.push_str(&http_res.response);
        res_with_body.push_str(body);
        match self.tcp_stream.write_all(res_with_body.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to write to TcpStream in respond with body!\n\t{}", e);
            }
        };
    }

    /// Generates HTTP request headers into Vec<LineOrError>
    fn gen_raw_req(mut stream: TcpStream) -> Result<(HttpHeaderBody, TcpStream), (String, TcpStream)> {
        let mut buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<LineOrError> = buf_reader
            .by_ref()
            .lines()
            .map(|result| match result {
                Ok(res) => LineOrError::Line(res),
                Err(error) => {
                    log::error!("Error reading line:\n\t{}", error);
                    LineOrError::Error(error.to_string())
                }
            })
        .take_while(|line| match line {
            LineOrError::Line(line) => !line.is_empty(),
            LineOrError::Error(_) => true,
        })
        .collect();
        let mut content_length = 0;
        let strang = "Content-Length:";
        for header in &http_request {
            if header.to_string().starts_with(strang) {
                let sub_head = &header.to_string()[(strang.len())..(header.to_string().len())]
                    .replace(' ', "");
                content_length = match sub_head.to_string().parse::<usize>() {
                    Ok(len) => len,
                    Err(e) => {
                        log::error!("{} Header Len {}", e, sub_head);
                        0
                    }
                };
                break;
            }
        }
        let mut body_buf = vec![0; content_length];
        log::debug!("Header Length {}", content_length);
        if let Err(e) = buf_reader.read_exact(&mut body_buf) {
            return Err((e.to_string(), stream));
        }
        let body: String = match String::from_utf8(body_buf) {
            Ok(bod) => bod,
            Err(e) => {
                return Err((e.to_string(), stream))
            }
        };
        let body_o = HttpHeaderBody::new(http_request, content_length, body);
        match body_o {
            Ok(body) => Ok((body, stream)),
            Err(reason) => Err((reason, stream)),
        }
    }

    fn gen_req_str(req: &Vec<LineOrError>) -> String {
        let mut req_string_mut: String = "".to_owned();

        for req_line in req {
            req_string_mut.push_str("\n\t");
            req_string_mut.push_str(req_line.to_string().as_str());
        }
        req_string_mut.push('\n');

        let req_string: String = req_string_mut.to_string();

        req_string
    }
}
