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

// Allows for equality comparisons between requests/responses
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


// Methods available for Request and Response objects. Uses builder pattern.
impl Request {

    // Creates a new request object with default values
    pub fn new() -> Request {
	let empty_header = HashMap::new();
        let request = Request { method: "".to_string(),
				route: "".to_string(),
				headers: empty_header
	};
	request
    }

// Request getters
    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    pub fn get_route(&self) -> String {
        self.method.clone()
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

// Request setters
    pub fn with_method(mut self, req_method: String) -> Request {
        self.method = req_method;
        self
    }

    pub fn with_route(mut self, res_route: String) -> Request {
        self.route = res_route;
        self
    }

    // Destroys current headers and replaces them with the arg
    pub fn with_headers(mut self, req_headers: HashMap<String, String>) -> Request {
        self.headers = req_headers;
        self
    }

    // Adds to existing hash map
    pub fn with_header(mut self, req_header: (String, String)) -> Request {
        self.headers.insert(req_header.0, req_header.1);
        self
    }
}


impl Response {
// Create a new response struct with default values
    pub fn new() -> Response {
        let response = Response { status: 0_i32,
				  content_type: "".to_string(),
				  body: "".to_string(),
				  headers: None
	};
	response
    }

// Response getters
    pub fn get_status(&self) -> i32 {
        self.status
    }

    pub fn get_content_type(&self) -> String {
        self.content_type.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn get_headers(&self) -> Option<HashMap<String, String>> {
        self.headers.clone()
    }

// Response setters
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

    // This will destroy all current headers and update with the arg
    pub fn with_headers(mut self, res_headers: Option<HashMap<String, String>>) -> Response {
        self.headers = res_headers;
        self
    }

    // Adds to the current header hashmap or creates a new one if necessary
    pub fn with_header(mut self, res_header: (String, String)) -> Response {
	match self.headers {
            Some(ref mut headers) => {
                headers.insert(res_header.0, res_header.1);
            },
            None => {
		let mut headers = HashMap::new();
		headers.insert(res_header.0, res_header.1);
		self.headers = Option::from(headers);
            }
        }
        self
    }

}
