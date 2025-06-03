use crate::http::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/home" => Response::new(StatusCode::Ok, Some("<h1>Home</h1>".to_string())),
                "/about" => Response::new(StatusCode::Ok, Some("<h1>About</h1>".to_string())),
                "/contact" => Response::new(StatusCode::Ok, Some("<h1>Contact</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, Some("<h1>404</h1>".to_string())),
            },
            _ => Response::new(
                StatusCode::NotFound,
                Some("<h1>404</h1><h1>Page Not Found</h1>".to_string()),
            ),
        }
    }
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    pub fn read_file(&self, path: &str) -> Option<String> {
        let path = format!("{}{}", self.public_path, path);
        fs::read_to_string(path).ok()
    }
}
