// currently there are conflicting issues with the compiler and clippy regarding static lifetimes
#![allow(clippy::redundant_static_lifetimes)]
pub mod server;
pub mod router;
pub mod http {
    pub mod request;
    pub mod response;
    pub mod shared;
    pub mod decoder;
}
#[macro_use]
extern crate lazy_static;

mod logger;

#[derive(Clone)]
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

