use std::collections::HashMap;

// User request
#[derive(Eq,Debug)]
pub struct Request {
    method : String,
    route : String,
    headers : HashMap<String, String>,
}

// Server response
#[derive(Eq,Debug)]
pub struct Response {
    status : i32,
    content_type : String,
    body : String,
    headers : Option<HashMap<String, String>>,
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

    /*
    Takes the raw request string and transforms it into a Request object.
    */
    pub fn get_request_obj(mut self, request : &str) -> Request {
        let request = request.trim_left();
        let lines = request.lines();
        let mut i = 0;
        let mut found_method : String = String::default();
        let mut found_route : String = String::default();
        let mut found_headers : HashMap<String, String> = HashMap::new();
        for line in lines {
            if i == 0 {
                let first_args : Vec<&str> = line.split_whitespace().collect();
                found_method = first_args[0].to_string();
                found_route = first_args[1].to_string();
                if found_route.ends_with("/") {
                    found_route.trim_right_matches("/");
                }
            }
            else {
                let pair : Vec<&str> = line.split(":").collect();
                if pair.len() > 1 {
                    let key = pair[0];
                    let value = pair[1].trim_left();
                    found_headers.insert(key.to_string(), value.to_string());
                }
            }
            i += 1;
        }

            self.method = found_method;
            self.route = found_route;
            self.headers = found_headers;
            self
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

    // Arg copied over as new header hashmap
    pub fn with_headers(mut self, req_headers: HashMap<String, String>) -> Request {
        self.headers = req_headers;
        self
    }

    // Adds to existing hash map
    pub fn with_header(mut self, req_header: (String, String)) -> Request {
        self.headers.insert(req_header.0, req_header.1);
        self
    }

    /*
    Takes a Request object and routes it via the method + ' ' + route
    to the appropriate user-defined function.
    TODO: we need to add the ability to pass along URL arguments to the
    user-defined function.
    */
    pub fn route_request<'a>(self) -> Response {
        let route_map = get_route_map();
        let mut route_request: String = self.method.clone();
        route_request.push(' ');
        route_request.push_str(&self.route.clone());
        let route_function = route_map.get(&route_request);
        match route_function {
            Some(x) => x(self),
            None => bad_route(),
        }
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

    // Sets headers to arg
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

    /*
    Takes a Response object and turns it into a single String that
    can be converted to a byte-stream and written back to the user.
    */
    pub fn stringify_response(&self) -> String {
        let mut res = String::from(format!("HTTP/1.1 {}\r\ncontent-type: {}\r\n",
                                            self.status,
                                            self.content_type));
        if self.headers.is_some() {
            let headers = self.get_headers().unwrap();
            for key in headers.keys() {
                res = res + &format!("{}: {}", key, headers[key]);
            }
        }
        res = res + "\r\n";
        res = res + &format!("{}", self.get_body());
        res = res + "\r\n\r\n";
        res
    }
}

// Static route with 404 status, used as default bad request
pub fn bad_route<'a>() -> Response {
    let route = Response::new().with_status(404)
                               .with_content_type("".to_string())
                               .with_body("Unable to route request".to_string())
                               .with_headers(None);
    route
}

/*
Gets the map of route Strings -> user-defined functions that take
a Request object and return a Response object.
*/
pub fn get_route_map() -> Box<HashMap<String, fn(Request) -> Response>> {
    let route_map : HashMap<String, fn(Request) -> Response> = HashMap::new();
    return Box::new(route_map);
}

/*
Takes a raw request String and transforms it into a Request object,
routes it to a user-defined function, transforms the return from that
function into a String and gives that back.
*/
pub fn get_http_response(request_string: &String) -> String {
    let request_obj = Request::new().get_request_obj(request_string);
    let response = request_obj.route_request();
    response.stringify_response()
}
