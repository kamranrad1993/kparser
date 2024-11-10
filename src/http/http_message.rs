use std::collections::hash_map::HashMap;

pub enum RequestMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

pub struct RequestStartLine {
    pub method: RequestMethod,
    pub path: String,
    pub version: String
}

pub struct HttpRequest {
    pub start_line: RequestStartLine,
    pub headers: HashMap<String, String>,
}

pub struct ResponseStartLine {
    pub version: String,
    pub response_code: u32,
    pub response_msg: String
}

pub struct HttpResponse {
    pub start_line: ResponseStartLine,
    pub headers: HashMap<String, String>,
}

pub enum HttpMessage{
    Request(HttpRequest),
    Response(HttpResponse)
}

