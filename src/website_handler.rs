use crate::http::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;
use std::path::Path;

pub struct WebsiteHandler {
    public_path: String,
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => self.serve_file("index.html"),
                "/home" => Response::new(StatusCode::Ok, Some("<h1>Home</h1>".to_string())),
                "/about" => Response::new(StatusCode::Ok, Some("<h1>About</h1>".to_string())),
                "/contact" => Response::new(StatusCode::Ok, Some("<h1>Contact</h1>".to_string())),
                path => {
                    // Remove leading slash and serve the file
                    let file_path = &path[1..];
                    self.serve_file(file_path)
                }
            },
            _ => Response::new(
                StatusCode::MethodNotAllowed,
                Some("<h1>405</h1><h1>Method Not Allowed</h1>".to_string()),
            ),
        }
    }
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn get_content_type(&self, file_path: &str) -> &str {
        match Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some("css") => "text/css",
            Some("js") => "text/javascript",
            Some("html") => "text/html",
            Some("png") => "image/png",
            Some("jpg") => "image/jpeg",
            Some("jpeg") => "image/jpeg",
            Some("svg") => "image/svg+xml",
            Some("woff") => "font/woff",
            Some("svg+xml") => "font/svg+xml",
            Some("woff2") => "font/woff2",
            Some("ttf") => "font/ttf",
            Some("json") => "application/json",
            _ => "text/plain",
        }
    }

    fn serve_file(&self, file_path: &str) -> Response {
        let path = Path::new(&self.public_path).join(file_path);

        // Ensure the path is within the public directory (prevent directory traversal)
        if let Ok(canonical_path) = path.canonicalize() {
            if let Ok(canonical_base) = Path::new(&self.public_path).canonicalize() {
                if canonical_path.starts_with(canonical_base) {
                    // Path is safe, try to read the file
                    match fs::read_to_string(&canonical_path) {
                        Ok(content) => {
                            let content_type = self.get_content_type(file_path);
                            let mut response = Response::new(StatusCode::Ok, Some(content));
                            response.content_type = Some(content_type.to_string());
                            return response;
                        }
                        Err(_) => return self.serve_file("not-found.html"),
                    }
                }
            }
        }

        // If path is unsafe or any error occurs, serve forbidden
        self.serve_error_page("not-found.html", StatusCode::MethodNotAllowed)
    }

    fn serve_error_page(&self, error_page: &str, status: StatusCode) -> Response {
        let path = Path::new(&self.public_path).join(error_page);
        match fs::read_to_string(path) {
            Ok(content) => Response::new(status, Some(content)),
            Err(_) => Response::new(status, Some(format!("<h1>{}</h1>", status.reason_phrase()))),
        }
    }
}
