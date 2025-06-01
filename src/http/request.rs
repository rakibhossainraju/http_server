use crate::http::{Method, ParseError};
use std::str;

#[derive(Debug)]
pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let raw_request = str::from_utf8(buffer)?;
        let (method, query_string) = Self::get_next_word(raw_request).ok_or(ParseError::InvalidRequest)?;
        let method = Method::from_str(method).ok_or(ParseError::InvalidMethod)?;


        Ok(Request {
            path: String::from("/"),
            method,
            query_string: if query_string == "" { None } else { Some(query_string.to_string()) },
        })
    }
}

impl Request {
    fn get_next_word(req_str: &str) -> Option<(&str, &str)> {
        for (idx, char) in req_str.chars().enumerate() {
            if char == ' ' {
                return Some((&req_str[..idx], &req_str[idx + 1..]));
            }
        }
        None
    }
}