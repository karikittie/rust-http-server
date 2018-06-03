pub mod content_type;

use std::collections::HashMap;
use self::content_type::CONTENT_TYPE;

// User request
#[derive(Eq,Debug)]
pub struct Request {
    method : String,
    route : String,
    headers : HashMap<String, String>,
    params : Vec<String>,
    args : HashMap<String, String>,
}

// Server response
//#[derive(Eq,Debug)]
pub struct Response {
    status : i32,
    content_type : CONTENT_TYPE,
    body : Vec<u8>,
    headers : Option<HashMap<String, String>>,
}

// Equality implementations for testing purposes

// Allows for equality comparisons between requests/responses
impl PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.method == other.method
        && self.route == other.route
        && self.headers == other.headers
        && self.params == other.params
        && self.args == other.args
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.status == other.status
        //&& self.content_type == other.content_type
        && self.body == other.body
        && self.headers == other.headers
    }
}

/// Builds a Response struct from a given body and content type with a status = 404
pub fn not_found<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    let response = Response::new().with_status(404)
                                  .with_content_type(content_type)
                                  .with_body(Vec::from(body.as_bytes()))
                                  .with_headers(None);
    response
}

/// Builds a Response struct from a given body and content type with a status = 200
pub fn ok<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    let response = Response::new().with_status(200)
                                  .with_content_type(content_type)
                                  .with_body(Vec::from(body.as_bytes()))
                                  .with_headers(None);
    response
}

/// Builds a Response struct from a given body (of Vec<u8>) and content type with a status = 200
pub fn ok_file<'a>(body: Vec<u8>, content_type: CONTENT_TYPE) -> Response {
    let response = Response::new().with_status(200)
                                  .with_content_type(content_type)
                                  .with_body(body)
                                  .with_headers(None);
    response
}

/// Builds a Response struct from a given body and content type with a status = 505
pub fn server_error<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    let response = Response::new().with_status(505)
                                  .with_content_type(content_type)
                                  .with_body(Vec::from(body.as_bytes()))
                                  .with_headers(None);
    response
}

/*
Takes the raw request string and transforms it into a Request object.
*/
impl Request {
    pub fn new() -> Request {
        Request {
            method: String::from("GET"),
            route: String::from(""),
            headers: HashMap::new(),
            params : Vec::new(),
            args : HashMap::new(),
        }
    }


    pub fn parse_query_string(mut self) -> Request {
        // Copy route from request object
        let route: String = self.get_route();
        // Split between path and query string
        let mut split: Vec<&str> = route.split("/").collect();
        // Take the right side and separate queries
        let mut queries: Vec<&str> = split.pop().unwrap().split("?").collect();
        // Create new string vector to collect parameters

        self.params_from_route(queries.clone()).args_from_route(queries.clone().as_mut())
    }


    // Finds params contained in route and includes them in the body
    // of the request object in the order found. Assumes that all params
    // will be located after the last hyphen of the path and end with a ?
    pub fn params_from_route(mut self, queries: Vec<&str>) -> Request {
        let mut params: Vec<String> = Vec::new();

        for i in queries {
            // Push anything that isn't an argument in the query string
            if i.contains("=") != true {
                params.push(i.to_string());
            }
        }
        self.params = params;
        self
    }

    // Pulls args from the route of the request object. Assumes that the first args
    // will be located after the last ? and all others will be after ampersands
    pub fn args_from_route(mut self, queries: &mut Vec<&str>) -> Request {
        //Create new hashmap to collect args
        let mut args: HashMap<String, String> = HashMap::new();
        // Separate arguments from params and separate
        let mut arg_section: Vec<&str> = queries.pop().unwrap().split("&").collect();

        // Find and split argument terms
        for i in arg_section {
                if i.contains("=") == true {
                let mut tuple: Vec<&str> = i.split("=").collect();
                args.insert(tuple[0].to_string(), tuple[1].to_string());
            }
        }

        self.args = args;
        self
    }

    /// Creates a Request object from a HTTP request.
    pub fn from(request : &str) -> Request {
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

        let new_request = Request::new().with_method(found_method)
                                        .with_route(found_route)
                                        .with_headers(found_headers)
                                        .with_params(Vec::new())
                                        .with_args(HashMap::new());
        new_request
    }

    // Request getters
    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    pub fn get_route(&self) -> String {
        let mut route = self.method.clone();
        let cloned_route = self.route.clone();
        route.push(' ');
        route.push_str(&cloned_route);
        route
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn get_params(&self) -> Vec<String> {
        self.params.clone()
    }

    pub fn get_args(&self) -> HashMap<String, String> {
        self.args.clone()
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
    pub fn with_params(mut self, req_params: Vec<String>) -> Request {
        self.params = req_params;
        self
    }

    pub fn with_param(mut self, req_param: String) -> Request {
        self.params.push(req_param);
        self
    }

    pub fn with_args(mut self, req_args: HashMap<String, String>) -> Request {
        self.args = req_args;
        self
    }

    pub fn with_arg(mut self, req_arg: (String, String)) -> Request {
        self.args.insert(req_arg.0, req_arg.1);
        self
    }
}

/*
Takes a Response object and turns it into a single String that
can be converted to a byte-stream and written back to the user.
*/
impl Response {
        // Create a new response struct with default values
        pub fn new() -> Response {
            Response { status: 0_i32,
    				   content_type: CONTENT_TYPE::TEXT_HTML,
    				   body: Vec::new(),
    				   headers: None
    	    }
        }

        fn stringify(&self) -> String {
            let mut res = String::from(format!("HTTP/1.1 {}\r\ncontent-type: {}\r\n",
                                                self.status,
                                                self.content_type.stringify()));
            if self.headers.is_some() {
                let headers = self.get_headers().unwrap();
                for key in headers.keys() {
                    res = res + &format!("{}: {}", key, headers[key]);
                }
            }
            res = res + "\r\n";
            res
        }

        pub fn byteify(&mut self) -> Vec<u8> {
            let part1 = self.stringify();
            let mut result: Vec<u8> = Vec::from(part1.as_bytes());
            result.append(&mut self.body);
            result
        }

        // Response getters
        pub fn get_status(&self) -> i32 {
            self.status.clone()
        }

        pub fn get_content_type(&self) -> CONTENT_TYPE {
            self.content_type.clone()
        }

        pub fn get_body(&self) -> Vec<u8> {
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

        pub fn with_content_type(mut self, res_content: CONTENT_TYPE) -> Response {
            self.content_type = res_content;
            self
        }

        pub fn with_body(mut self, res_body: Vec<u8>) -> Response {
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
}

/*
Takes a Request object and routes it via the method + ' ' + route
to the appropriate user-defined function.
TODO: we need to add the ability to pass along URL arguments to the
user-defined function.
*/
pub fn route_request<'a>(request : Request) -> Response {
    let route_map = get_route_map();
    let mut route_request: String = request.method.clone();
    route_request.push(' ');
    route_request.push_str(&request.route);
    let route_function = route_map.get(&route_request);
    match route_function {
        Some(x) => x(request),
        None => server_error(String::from("Unable to route request"), CONTENT_TYPE::TEXT_HTML),
    }
}

/*
Gets the map of route Strings -> user-defined functions that take
a Request object and return a Response object.
*/
fn get_route_map() -> Box<HashMap<String, fn(Request) -> Response>> {
    let route_map : HashMap<String, fn(Request) -> Response> = HashMap::new();
    Box::new(route_map)
}
