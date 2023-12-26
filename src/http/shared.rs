use std::collections::HashMap;

use crate::LineOrError;

use super::decoder::HttpUrlDecoder;

#[derive(Copy, Clone, PartialEq, Eq)]
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
        let mut boundary: Option<String> = None;
        for line_or_error in &lines {
            match line_or_error {
                LineOrError::Line(line) => {
                    if line.starts_with("Content-Type: ") {
                        let content_type = line.trim_start_matches("Content-Type: ").trim();
                        body_type = match content_type {
                            ct if ct.starts_with(HttpBodyType::FormData.to_str()) => {
                                Some(HttpBodyType::FormData)
                            },
                            ct if ct.starts_with(HttpBodyType::UrlEncoded.to_str()) => Some(HttpBodyType::UrlEncoded),
                            _ => None,
                        };
                        if body_type.is_none() {
                            let mut message = "Bad request! Unrecognized Content-Type: ".to_string();
                            message.push_str(content_type);
                            return Err(message);
                        }
                        if let Some(bod) = body_type {
                            if bod == HttpBodyType::FormData {
                                boundary = Some(content_type.trim_start_matches(HttpBodyType::FormData.to_str()).trim().trim_start_matches("; boundary=").trim().to_string())
                            }
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
                    body_params: Self::gen_params(body_str, body, boundary),
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

    fn gen_params(body: String, body_type: HttpBodyType, boundary: Option<String>) -> Option<HashMap<String, String>> {
        if body.len() <= 0 {
            return None;
        }
        match body_type {
            HttpBodyType::FormData => {
                Some(Self::gen_params_form_data(body, boundary))
            },
            HttpBodyType::UrlEncoded => {
                Some(Self::gen_params_url_encoded(body))
            }
        }
    }

    fn gen_params_form_data(body: String, boundary: Option<String>) -> HashMap<String, String> {
        let mut params = HashMap::new();
        match boundary {
            Some(bound) => {
                let mut was_boundary: bool = false;
                let mut bad_boundary: bool = false;
                let mut key = String::new();
                let mut value = String::new();
                for key_value in body.split("\n") {
                    // we have entered in bound
                    if key_value.contains(&bound) {
                        if key.len() > 0 {
                            params.insert(key, value);
                        }
                        was_boundary = true;
                        bad_boundary = false;
                        key = String::new();
                        value = String::new();
                    }
                    // should be content-disp line
                    else if was_boundary {
                       was_boundary = false; 
                       log::debug!("key-value: {}", key_value);
                       match key_value.trim_start_matches("Content-Disposition: form-data; name=\"") {
                           kv if kv.len() > 0 => {
                               let char_count: usize = kv.chars().count() - 2;
                               key = kv.chars().take(char_count).collect::<String>();
                           },
                           _ => {
                               log::warn!("Bad boundary in FormData request. Parameters may be missing!");
                               bad_boundary = true
                           },
                       }
                    }
                    // make sure its not just an empty line
                    else if !bad_boundary && key_value.len() > 1 {
                        // if value is len is greater than 0 that means we need to add a \n
                        if value.len() > 0 {
                            value.push_str("\n");
                        }
                        value.push_str(key_value);
                    }
                }
            },
            None => (),
        }
        println!(" ");
        println!(" ");
        println!(" ");
        // iterate over hashmap and just print out the key value pairs
        for (key, value) in &params {
            log::debug!("{}: {}", key, value);
        }
        println!(" ");
        println!(" ");
        println!(" ");
        params
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
