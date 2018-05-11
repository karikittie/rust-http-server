use std::collections::HashMap;

// User request
#[derive(Eq,Debug)]
pub struct Request {
    pub method : String,
    pub route : String,
    pub headers : HashMap<String, String>,
}

// Server response
#[derive(Eq,Debug)]
pub struct Response {
    pub status : i32,
    pub content_type : String,
    pub body : String,
    pub headers : Option<HashMap<String, String>>,
}


impl PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.method == other.method
        && self.route == other.route
        && self.headers == other.headers
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.status == other.status
        && self.content_type == other.content_type
        && self.body == other.body
        && self.headers == other.headers
    }
}


// Methods available for Request and Response objects. Uses builder pattern for ease of access.
impl Request {

    pub fn with_method(mut self, req_method: String) -> Request {
        self.method = req_method;
        self
    }

    pub fn with_route(mut self, res_route: String) -> Request {
        self.route = res_route;
        self
    }

    pub fn with_headers(mut self, req_headers: HashMap<String, String>) -> Request {
        self.headers = req_headers;
        self
    }

}


impl Response {

    pub fn with_status(mut self, res_status: i32) -> Response {
        self.status = res_status;
        self
    }

    pub fn with_content_type(mut self, res_content: String) -> Response {
        self.content_type = res_content;
        self
    }

    pub fn with_body(mut self, res_body: String) -> Response {
        self.body = res_body;
        self
    }

    pub fn with_headers(mut self, res_headers: Option<HashMap<String, String>>) -> Response {
        self.headers = res_headers;
        self
    }

}
