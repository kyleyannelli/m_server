use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader, self}
};

use log::{info, warn, error};

use rust_link::{HttpResponse, LineOrError};

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

    let listener = match TcpListener::bind(self::BIND_ADDR) {
        Ok(lis) => lis,
        Err(error) => match error.kind() {
            io::ErrorKind::AddrInUse => {
                error!("Address {} already in use. Please make sure an instance is not already running, or no other services use the port.", self::BIND_ADDR);
                std::process::exit(1);
            },
            _ => {
                error!("Error occurred while binding: {} ", error.to_string());
                std::process::exit(1);
            }
        }
    };

    info!("{} {}", "Server bound on", self::BIND_ADDR);

    for stream_res in listener.incoming() {
        match stream_res {
            Ok(result) => self::handle_conn(result),
            Err(error) => match error.kind() {
                io::ErrorKind::WouldBlock => {
                    warn!("Waiting for network socket to be ready");
                    continue;
                },
                _ => {
                    let err: String = error.to_string();
                    error!("{}", err);
                }
            }
        }
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<LineOrError> = buf_reader
        .lines()
        .map(|result| match result {
                Ok(res) => LineOrError::Line(res),
                Err(error) => {
                    error!("Error reading line:\n\t{}", error);
                    LineOrError::Error(error.to_string())
                },
        })
        .take_while(|line| match line {
            LineOrError::Line(line) => !line.is_empty(),
            LineOrError::Error(_) => true,
        })
        .collect();

    for req_line in _http_request {
        println!("{}", req_line);
    }
    println!();

    stream.write_all(HttpResponse::ok().as_bytes()).unwrap();
}

