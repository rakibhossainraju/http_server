use std::io::Read;
use std::net::TcpListener;

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
                        Ok(size) => {
                            println!("Received connection from {}", String::from_utf8_lossy(&buffer));
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
