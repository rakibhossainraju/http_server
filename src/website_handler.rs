use crate::http::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path(){
                "/" => Response::new(StatusCode::Ok, Some("<h1>Home</h1>".to_string())),
                "/home" => Response::new(StatusCode::Ok, Some("<h1>Home</h1>".to_string())),
                "/about" => Response::new(StatusCode::Ok, Some("<h1>About</h1>".to_string())),
                "/contact" => Response::new(StatusCode::Ok, Some("<h1>Contact</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, Some("<h1>404</h1>".to_string()))
            },
            _ => Response::new(StatusCode::NotFound, Some("<h1>404</h1><h1>Page Not Found</h1>".to_string()))
        }
    }
}