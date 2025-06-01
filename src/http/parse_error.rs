use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidPath,
    InvalidQueryString,
    InvalidProtocol,
    InvalidEncoding,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ParseError: {}", self.message())
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request format",
            Self::InvalidMethod => "Invalid HTTP method",
            Self::InvalidPath => "Invalid request path",
            Self::InvalidQueryString => "Invalid query string format",
            Self::InvalidProtocol => "Invalid HTTP protocol version",
            Self::InvalidEncoding => "Invalid encoding in request",
        }
    }
}