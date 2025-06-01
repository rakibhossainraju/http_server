use crate::http::Method;

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidPath,
    InvalidQueryString,
    InvalidProtocol,
}


pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}