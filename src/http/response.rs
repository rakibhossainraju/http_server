use crate::http::StatusCode;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::{Shutdown, TcpStream};

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub headers: Vec<(String, String)>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
            content_type: Some("text/html".to_string()),
            headers: vec![],
        }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");
        let content_type = self.content_type.as_deref().unwrap_or("text/html");

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\n",
            self.status_code,
            self.status_code.reason_phrase(),
            content_type,
        )?;

        for (name, value) in &self.headers {
            write!(stream, "{}: {}\r\n", name, value)?;
        }

        write!(stream, "Content-Length: {}\r\n\r\n", body.len())?;
        write!(stream, "{}", body)?;

        stream.flush()?;
        stream.shutdown(Shutdown::Write)
    }

    pub fn add_header(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers;
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = self.body.as_deref().unwrap_or("");
        let content_type = self.content_type.as_deref().unwrap_or("text/html");

        write!(
            f,
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            content_type,
            body.len(),
            body
        )
    }
}
