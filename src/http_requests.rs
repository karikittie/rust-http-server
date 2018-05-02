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

pub fn BadRoute<'a>() -> Response {
    let content_type = String::from("");
    let body = String::from("Unable to route request");
    return Response {status : 404,
                     content_type : content_type,
                     body : body,
                     headers : None};
}

fn get_route_map() -> HashMap<String, fn(Request) -> Response> {
    let route_map : HashMap<String, fn(Request) -> Response> = HashMap::new();
    return route_map;
}

pub fn get_request_obj(request : &str) -> Request {
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

pub fn route_request<'a>(request : &Request) -> Response {
    return BadRoute();
}

pub fn stringify_response(response : &Response) -> String {
    let mut res = String::from(format!("HTTP/1.1 {}\r\ncontent-type: {}\r\n", 
                                        response.status, 
                                        response.content_type));
    if response.headers.is_some() {
        let headers = response.headers.as_ref().unwrap();
        for key in headers.keys() {
            res = res + &format!("{}: {}", key, headers[key]);
        }
    }
    res = res + "\r\n";
    res = res + &format!("{}", response.body);
    res = res + "\r\n\r\n";
    res
}

pub fn get_http_response(request_string: &String) -> String {
    stringify_response(&route_request(&get_request_obj(request_string.as_str())))
}
