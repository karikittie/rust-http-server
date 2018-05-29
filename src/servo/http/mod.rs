pub mod content_type;

use std::collections::HashMap;
use self::content_type::CONTENT_TYPE;

pub struct Request {
    method : String,
    route : String,
    headers : HashMap<String, String>,
}

pub struct Response {
    status : i32,
    content_type : CONTENT_TYPE,
    body : Vec<u8>,
    headers : Option<HashMap<String, String>>,
}

impl Request {
    pub fn get_route(&self) -> String {
        let mut route = self.method.clone();
        let cloned_route = self.route.clone();
        route.push(' ');
        route.push_str(&cloned_route);
        route
    }
}

/// Builds a Response struct from a given body and content type with a status = 404
pub fn not_found<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    return Response {status : 404,
                    content_type : content_type,
                    body : Vec::from(body.as_bytes()),
                    headers : None};
}

/// Builds a Response struct from a given body and content type with a status = 200
pub fn ok<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    Response {
        status : 200,
        content_type : content_type,
        body : Vec::from(body.as_bytes()),
        headers : None
    }
}

/// Builds a Response struct from a given body (as Vec<u8>) and content type with a status = 200
pub fn ok_file<'a>(body: Vec<u8>, content_type: CONTENT_TYPE) -> Response {
    Response {
        status : 200,
        content_type : content_type,
        body : body,
        headers : None
    }
}

/// Builds a Response struct from a given body and content type with a status = 505
pub fn server_error<'a>(body: String, content_type: CONTENT_TYPE) -> Response {
    Response {
        status : 505,
        content_type : content_type,
        body : Vec::from(body.as_bytes()),
        headers : None
    }
}

/*
Takes the raw request string and transforms it into a Request object.
*/
impl Request {
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
        let new_request = Request {method : found_method, 
                                   route : found_route,
                                   headers : found_headers};
        new_request
    }
}

/*
Takes a Response object and turns it into a single String that
can be converted to a byte-stream and written back to the user.
*/
impl Response {
    fn stringify(&self) -> String {
        let mut res = String::from(format!("HTTP/1.1 {}\r\ncontent-type: {}\r\n", 
                                            self.status, 
                                            self.content_type.stringify()));
        if self.headers.is_some() {
            let headers = self.headers.as_ref().unwrap();
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
}