pub struct HttpResponse;

impl HttpResponse {
    const HTTP_VER: &'static str = "HTTP/1.1";
    const HTTP_PAD: &'static str = "\r\n\r\n";
    const RESPONSE_OK: &'static str = "200 OK";
    const RESPONSE_NOT_FOUND: &'static str = "404 Not Found";
    const RESPONSE_ERROR: &'static str = "500 Internal Server Error";
    const RESPONSE_CREATED: &'static str = "201 Created";
    const RESPONSE_ACCEPTED: &'static str = "Accepted";

    fn status_message(status_code: &str) -> String {
        return format!("{} {}{}", Self::HTTP_VER, status_code, Self::HTTP_PAD);
    }

    pub fn ok() -> String {
        return Self::status_message(Self::RESPONSE_OK);
    }

    pub fn not_found() -> String {
        return Self::status_message(Self::RESPONSE_NOT_FOUND);
    }

    pub fn created() -> String {
        return Self::status_message(Self::RESPONSE_CREATED);
    } 

    pub fn accepted() -> String {
        return Self::status_message(Self::RESPONSE_ACCEPTED);
    }
    
    pub fn error() -> String {
        return Self::status_message(Self::RESPONSE_ERROR);
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

