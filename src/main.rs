mod http;
use http::Server;

fn main() {
    let server = Server::new("127.0.0.1".to_string(), 8080);
    server.start();
    server.stop();
}
