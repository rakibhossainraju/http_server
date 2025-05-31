pub struct Server {
    address: String,
    port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Server { address, port }
    }

    pub fn start(&self) {
        println!("Starting server at {}:{}", self.address, self.port);
    }
    pub fn stop(self) {
        println!("Stopping server at {}:{}", self.address, self.port);
    }
}
