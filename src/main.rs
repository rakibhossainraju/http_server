#![allow(dead_code)]
mod http;
mod website_handler;

use http::Server;
use website_handler::WebsiteHandler;

fn main() {
    let public_path = env!("CARGO_MANIFEST_DIR").to_string() + "/public/";
    let server = Server::new("127.0.0.1".to_string(), 8080);
    server.start(WebsiteHandler::new(public_path));
    // server.stop();
}
