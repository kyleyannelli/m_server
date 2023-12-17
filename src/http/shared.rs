use crate::LineOrError;

pub enum HttpBodyType {
    FormData,
    UrlEncoded,
}

impl HttpBodyType {
    fn to_str(&self) -> &'static str {
        match self {
            HttpBodyType::FormData => "multipart/form-data",
            HttpBodyType::UrlEncoded => "application/x-www-form-urlencoded",
        }
    }
}

pub struct HttpHeaderBody {
    pub lines: Vec<LineOrError>,
    pub header_len: usize, 
    pub body_type: HttpBodyType,
}

impl HttpHeaderBody {
    pub fn new(lines: Vec<LineOrError>, header_len: usize) -> Result<HttpHeaderBody, String> {
        let mut body_type: Option<HttpBodyType> = None;
        for line_or_error in &lines {
            match line_or_error {
                LineOrError::Line(line) => {
                    if line.starts_with("Content-Type: ") {
                        let content_type = line.trim_start_matches("Content-Type: ").trim();
                        body_type = match content_type {
                            ct if ct.starts_with(HttpBodyType::FormData.to_str()) => Some(HttpBodyType::FormData),
                            ct if ct.starts_with(HttpBodyType::UrlEncoded.to_str()) => Some(HttpBodyType::UrlEncoded),
                            _ => None,
                        };
                        if body_type.is_none() {
                            let mut message = "Bad request! Unrecognized Content-Type: ".to_string();
                            message.push_str(content_type);
                            return Err(message);
                        }
                        break;
                    }
                },
                LineOrError::Error(error) => {
                    log::error!("Error parsing header! {}", error);
                    return Err(error.to_string());
                }
            }
        }

        Ok(HttpHeaderBody {
            lines,
            header_len,
            body_type: body_type.unwrap(),
        })
    }
}
