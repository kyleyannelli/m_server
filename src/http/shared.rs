use std::collections::HashMap;

use crate::LineOrError;

use super::decoder::HttpUrlDecoder;

#[derive(Copy, Clone)]
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
    pub body_type: Option<HttpBodyType>,
    pub body_params: Option<HashMap<String, String>>,
}

impl HttpHeaderBody {
    pub fn new(lines: Vec<LineOrError>, header_len: usize, body_str: String) -> Result<HttpHeaderBody, String> {
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

        match body_type {
            Some(body) => {
                Ok(HttpHeaderBody {
                    lines,
                    header_len,
                    body_type: Some(body),
                    body_params: Self::gen_params(body_str, body),
                })
            },
            None => {
                Ok(HttpHeaderBody {
                    lines,
                    header_len,
                    body_type: None,
                    body_params: None,
                })
            }
        }
    }

    fn gen_params(body: String, body_type: HttpBodyType) -> Option<HashMap<String, String>> {
        if body.len() <= 0 {
            return None;
        }
        match body_type {
            HttpBodyType::FormData => {
                None
            },
            HttpBodyType::UrlEncoded => {
                Some(Self::gen_params_url_encoded(body))
            }
        }
    }

    fn gen_params_url_encoded(body: String) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        for key_value in body.split('&') {
            let mut k_v = key_value.split('=');
            if let (Some(key), Some(value)) = (k_v.next(), k_v.next()) {
                params.insert(HttpUrlDecoder::decode_utf_8(key), HttpUrlDecoder::decode_utf_8(value));
            }
            else {
                log::warn!("Received intended key value in UrlEncoded body, but was unexpected.");
            }
        }
        params
    }
}
