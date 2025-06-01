use crate::http::{Method, ParseError};
use std::str;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: Method,
    pub query_string: Option<String>,
}

enum AllowedProtocol {
    HTTP1_1,
}
impl AllowedProtocol {
    fn from_str(name: &str) -> Option<AllowedProtocol> {
        match name {
            "HTTP/1.1" => Some(AllowedProtocol::HTTP1_1),
            _ => None,
        }
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let raw_request = str::from_utf8(buffer)?;

        let (method, rest) = Self::get_next_word(raw_request).ok_or(ParseError::InvalidRequest)?;
        let method = Method::from_str(method).ok_or(ParseError::InvalidMethod)?;
        let (path, rest) = Self::get_next_word(rest).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = Self::get_next_word(rest).ok_or(ParseError::InvalidRequest)?;
        AllowedProtocol::from_str(protocol).ok_or(ParseError::InvalidProtocol)?;


        Ok(Request {
            path: path.to_string(),
            method,
            query_string: None,
        })
    }
}

impl Request {
    fn get_next_word(req_str: &str) -> Option<(&str, &str)> {
        for (idx, char) in req_str.chars().enumerate() {
            if char == ' ' || char == '\r' || char == '\n' {
                return Some((&req_str[..idx], &req_str[idx + 1..]));
            }
        }
        None
    }
}