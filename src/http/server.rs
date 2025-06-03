use crate::http::response::Response;
use crate::http::{Request, StatusCode};
use std::error::Error;
use std::io::{Error as IoError, Read};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
    port: u16,
}

pub trait Handler {
    fn handle_request(&self, _: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("Default Implementation".to_string()))
    }
    fn handle_bad_request(&self, err: &dyn Error) -> Response {
        Response::new(StatusCode::BadRequest, Some(err.to_string()))
    }
    fn handle_not_found(&self, err: &dyn Error) -> Response {
        Response::new(StatusCode::NotFound, Some(err.to_string()))
    }
    fn handle_internal_server_error(&self, err: &dyn Error) -> Response {
        Response::new(StatusCode::InternalServerError, Some(err.to_string()))
    }
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Server { address, port }
    }

    pub fn start(&self, handler: impl Handler) {
        let address = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(address).unwrap();
        println!("Server started at http://localhost:{}", self.port);

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    if self.handle_connection(&handler, stream).is_err() {
                        eprintln!("Failed to handle connection");
                    };
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    pub fn stop(self) {
        println!("Stopping server at {}", self.address);
    }

    fn handle_connection(
        &self,
        handler: &impl Handler,
        mut stream: TcpStream,
    ) -> Result<(), IoError> {
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        let response = match Request::try_from(&buffer[..size]) {
            Ok(request) => handler.handle_request(&request),
            Err(err) => handler.handle_internal_server_error(&err),
        };
        response.send(&mut stream)?;
        Ok(())
    }
}
