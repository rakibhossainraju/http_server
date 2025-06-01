#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl Method {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "PATCH" => Some(Self::PATCH),
            "HEAD" => Some(Self::HEAD),
            "OPTIONS" => Some(Self::OPTIONS),
            "TRACE" => Some(Self::TRACE),
            "CONNECT" => Some(Self::CONNECT),
            _ => None,
        }
    }
}