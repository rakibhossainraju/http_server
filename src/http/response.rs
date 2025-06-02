use crate::http::StatusCode;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::{Shutdown, TcpStream};

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        write!(stream,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               self.body.as_deref().unwrap_or("")
        )
            .and_then(|_| stream.flush())
            .and_then(|_| stream.shutdown(Shutdown::Write))
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               self.body.as_deref().unwrap_or("")
        )
    }
}