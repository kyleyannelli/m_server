const HEADER_SERVER_NAME: &'static str = "Server: m_server/0.1.4-alpha";
const HEADER_CONTENT_TYPE: &'static str = "Content-Type: application/json";

pub struct HttpResponse {
    pub response: String,
}

impl HttpResponse {
    const HTTP_VER: &'static str = "HTTP/1.1";
    const HTTP_PAD: &'static str = "\r\n\r\n";
    const RESPONSE_OK: &'static str = "200 OK";
    const RESPONSE_NOT_FOUND: &'static str = "404 Not Found";
    const RESPONSE_ERROR: &'static str = "500 Internal Server Error";
    const BAD_REQUEST: &'static str = "400 Bad Request";
    const RESPONSE_CREATED: &'static str = "201 Created";
    const RESPONSE_ACCEPTED: &'static str = "Accepted";
    
    pub fn new(response: String) -> HttpResponse {
        HttpResponse {
            response,
        }
    }

    pub fn bad_request() -> HttpResponse {
        let response_str: String = Self::status_message(Self::BAD_REQUEST);
        HttpResponse::new(response_str)
    }

    pub fn ok() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_OK);
        HttpResponse::new(response_str)
    }

    pub fn not_found() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_NOT_FOUND);
        HttpResponse::new(response_str)
    }

    pub fn created() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_CREATED);
        HttpResponse::new(response_str)
    }

    pub fn accepted() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_ACCEPTED);
        HttpResponse::new(response_str)
    }

    pub fn error() -> HttpResponse {
        let response_str: String = Self::status_message(Self::RESPONSE_ERROR);
        HttpResponse::new(response_str)
    }

    fn status_message(status_code: &str) -> String {
        format!("{} {}\n{}\n{}{}", Self::HTTP_VER, status_code, self::HEADER_SERVER_NAME, self::HEADER_CONTENT_TYPE, Self::HTTP_PAD)
    }
}
