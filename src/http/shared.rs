use std::{collections::HashMap, net::TcpStream, io::{BufReader, Read}, str::from_utf8};

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
    pub fn new(lines: Vec<LineOrError>, buf_reader: BufReader<&mut TcpStream>, header_len: usize) -> Result<HttpHeaderBody, String> {
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
                    body_params: Self::gen_params(buf_reader, header_len, body, boundary),
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

    fn gen_params(buf_reader: BufReader<&mut TcpStream>, header_len: usize, body_type: HttpBodyType, boundary: Option<String>) -> Option<HashMap<String, String>> {
        if header_len == 0 {
            return None;
        }
        match body_type {
            HttpBodyType::FormData => {
                Some(Self::gen_params_form_data(buf_reader, header_len, boundary))
            },
            HttpBodyType::UrlEncoded => {
                Some(Self::gen_params_url_encoded(buf_reader, header_len))
            }
        }
    }

    fn gen_params_form_data(mut buf_reader: BufReader<&mut TcpStream>, header_len: usize, boundary: Option<String>) -> HashMap<String, String> {
            let mut buffer = Vec::new();
            let mut key = String::new();
            let mut value = String::new();
            let mut in_boundary = false;
            let mut bad_boundary = false;
            let mut params: HashMap<String, String> = HashMap::new();

            if let Some(bound) = boundary {
                for _ in 0..header_len {
                    let mut byte = [0];
                    if buf_reader.read_exact(&mut byte).is_err() {
                        // Handle read error or end of stream
                        break;
                    }

                    // Check for newline to process the buffered line
                    if byte[0] == b'\n' {
                        let line = from_utf8(&buffer).unwrap_or_default().trim().to_string();

                        if line.contains(&bound) {
                            if !key.is_empty() {
                                params.insert(key.clone(), value.clone());
                            }
                            in_boundary = true;
                            bad_boundary = false;
                            key.clear();
                            value.clear();
                        } else if in_boundary {
                            in_boundary = false;
                            if let Some(kv) = line.trim_start_matches("Content-Disposition: form-data; name=\"").split_once('"') {
                                key = kv.0.to_string();
                            } else {
                                bad_boundary = true;
                            }
                        } else if !bad_boundary && !line.is_empty() {
                            if !value.is_empty() {
                                value.push('\n');
                            }
                            value.push_str(&line);
                        } else if !bad_boundary {
                            if !value.is_empty() {
                                value.push('\n');
                            }
                        }

                        buffer.clear();
                    } else {
                        buffer.push(byte[0]);
                    }
                }
            }

            // Insert the last key-value pair if there is one
            if !key.is_empty() && !value.is_empty() {
                params.insert(key, value);
            }

        params
    }

    fn gen_params_url_encoded(mut buf_reader: BufReader<&mut TcpStream>, header_len: usize) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        let mut key = String::new();
        let mut value = String::new();
        let mut is_key = true;

        for _ in 0..header_len {
            let mut byte = [0];
            if buf_reader.read_exact(&mut byte).is_err() {
                log::error!("Failed to read byte from TcpStream!");
                break;
            }

            match byte[0] {
                b'=' => {
                    is_key = false;
                },
                b'&' => {
                    params.insert(HttpUrlDecoder::decode_utf_8(&key), HttpUrlDecoder::decode_utf_8(&value));
                    key.clear();
                    value.clear();
                    is_key = true;
                },
                _ => {
                    if is_key {
                        key.push(byte[0] as char);
                    }
                    else {
                        value.push(byte[0] as char);
                    }
                }
            }
        }

        if !key.is_empty() && !value.is_empty() {
            params.insert(HttpUrlDecoder::decode_utf_8(&key), HttpUrlDecoder::decode_utf_8(&value));
        }

        params
    }
}
