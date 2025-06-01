use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;

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
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {}
                                Err(error) => {
                                    println!("Failed to parse request: {:?}", error);
                                }
                            }
                        }
                        Err(err) => {
                            println!("Failed to read from stream: {}", err);
                        }
                    }
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
}
