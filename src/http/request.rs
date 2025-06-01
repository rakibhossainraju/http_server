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
        match str::from_utf8(buffer) {
            Ok(raw_request) => {
                Ok(Request {
                    path: String::from("/"),
                    method: Method::GET,
                    query_string: None,
                })
            }
            Err(_) => Err(ParseError::InvalidEncoding)
        }
    }
}