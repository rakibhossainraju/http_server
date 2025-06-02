use crate::http::response::Response;
use crate::http::{Request, StatusCode};
use std::io::{Error as IoError, Read};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
    port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Server { address, port }
    }

    pub fn start(&self) {
        let address = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(address).unwrap();
        println!("Server started at http://localhost:{}", self.port);

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    if self.handle_connection(stream).is_err() {
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

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), IoError> {
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer)?;
        let response = self.process_request(&buffer[..size]);
        response.send(&mut stream)?;
        Ok(())
    }

    fn process_request(&self, buffer: &[u8]) -> Response {
        match Request::try_from(buffer) {
            Ok(request) => {
                Response::new(StatusCode::Ok, Some("<h1>Hello World</h1>".to_string()))
            }
            Err(error) => {
                Response::new(StatusCode::InternalServerError, Some(error.to_string()))
            }
        }
    }
}
