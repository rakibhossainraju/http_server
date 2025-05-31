enum Method {
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

pub struct Request {
    path: String,
    method: Method,
    query_string: Option<String>,
}
