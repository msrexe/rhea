pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,    
}

impl Method {
    pub fn clone(&self) -> Self {
        match self {
            Method::GET => Method::GET,
            Method::POST => Method::POST,
            Method::PUT => Method::PUT,
            Method::DELETE => Method::DELETE,
            Method::HEAD => Method::HEAD,
            Method::CONNECT => Method::CONNECT,
            Method::OPTIONS => Method::OPTIONS,
            Method::TRACE => Method::TRACE,
            Method::PATCH => Method::PATCH,
        }
    }

    pub fn from_str(method: &str) -> Result<Method, &'static str> {
        match method {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err("Invalid method"),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: String,
    pub body: String,
}

impl Request {
    pub fn new(method: Method, path: String, headers: String) -> Self {
        Self {
            method,
            path,
            headers: headers,
            body: String::new(),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            path: self.path.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }
}   
