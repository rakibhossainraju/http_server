use crate::http::response::Response;
use crate::http::{Request, StatusCode};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

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
                    self.handle_connection(stream);
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

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let response = self.process_request(&buffer[..size]);
                self.write_and_close_stream(&mut stream, response);
            }
            Err(err) => {
                let response = Response::new(StatusCode::InternalServerError, Some(err.to_string()));
                self.write_and_close_stream(&mut stream, response);
            }
        }
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
    fn write_and_close_stream(&self, stream: &mut TcpStream, response: Response) {
        if let Err(err) = write!(stream, "{}", response)
            .and_then(|_| stream.flush())
            .and_then(|_| stream.shutdown(Shutdown::Write))
        {
            eprintln!("Failed to write response: {}", err);
        }
    }
}
