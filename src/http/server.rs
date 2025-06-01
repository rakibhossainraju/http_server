use crate::http::Request;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Server { address: address + ":" + &port.to_string() }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
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
                self.process_request(&buffer[..size]);
            }
            Err(err) => {
                println!("Failed to read from stream: {}", err);
            }
        }
    }
    fn process_request(&self, buffer: &[u8]) {
        match Request::try_from(buffer) {
            Ok(request) => {
                dbg!(request.path, request.method, request.query_string);
            }
            Err(error) => {
                println!("Failed to parse request: {:?}", error);
            }
        };
    }
}
