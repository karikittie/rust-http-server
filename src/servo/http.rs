use std::collections::HashMap;

pub struct Request {
    method : String,
    route : String,
    headers : HashMap<String, String>,
}

pub struct Response {
    status : i32,
    content_type : String,
    body : String,
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

// Static route with 404 status, used as default bad request
pub fn bad_route<'a>() -> Response {
    let content_type = String::from("text/html");
    let body = String::from("Unable to route request");
    return Response {status : 404,
                    content_type : content_type,
                    body : body,
                    headers : None};
}

pub fn ok<'a>(body: String) -> Response {
    let content_type = String::from("text/html");
    Response {
        status : 200,
        content_type : content_type,
        body : body,
        headers : None
    }
}

pub fn not_found<'a>(body: String) -> Response {
    let content_type = String::from("text/html");
    Response {
        status: 404,
        content_type : content_type,
        body : body,
        headers : None
    }
}

/*
Gets the map of route Strings -> user-defined functions that take
a Request object and return a Response object.
*/
fn get_route_map() -> Box<HashMap<String, fn(Request) -> Response>> {
    let route_map : HashMap<String, fn(Request) -> Response> = HashMap::new();
    return Box::new(route_map);
}

/*
Takes the raw request string and transforms it into a Request object.
*/
impl Request {
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
        return new_request;
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
        None => bad_route(),
    }
}

/*
Takes a Response object and turns it into a single String that
can be converted to a byte-stream and written back to the user.
*/
impl Response {
    pub fn stringify(self) -> String {
        let mut res = String::from(format!("HTTP/1.1 {}\r\ncontent-type: {}\r\n", 
                                            self.status, 
                                            self.content_type));
        if self.headers.is_some() {
            let headers = self.headers.as_ref().unwrap();
            for key in headers.keys() {
                res = res + &format!("{}: {}", key, headers[key]);
            }
        }
        res = res + "\r\n";
        res = res + &format!("{}", self.body);
        res = res + "\r\n\r\n";
        res
    }
}