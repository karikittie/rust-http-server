extern crate servo;
use servo::http::{Request, Response};
use servo::http::content_type::ContentType;
use std::collections::HashMap;


#[test]
fn request_eq_test(){
    let method = "method".to_string();
    let route = "route".to_string();
    let mut map = HashMap::new();
    map.insert("test".to_string(), "header".to_string());

    let original = Request::new().with_method(method.clone())
                                 .with_route(route.clone())
                                 .with_headers(map.clone());
    let request = Request::new().with_method(method)
                                .with_route(route)
                                .with_headers(map);
    assert_eq!(true, original == request)
}

#[test]
fn response_eq_test() {
    let status = 0_i32;
    let content = ContentType::TextHtml;
    let body = b"body";
    let mut map = HashMap::new();
    map.insert("test".to_string(), "header".to_string());

    let original = Response::new().with_status(status)
				                  .with_content_type(content.clone())
				                  .with_body(body.clone().to_vec())
				                  .with_headers(Option::from(map.clone()));
    let response = Response::new().with_status(status)
				                  .with_content_type(content)
				                  .with_body(body.to_vec())
				                  .with_headers(Option::from(map));
    assert_eq!(true, original == response)
}

#[test]
fn request_neq_test() {
    let original = Request::new();
    let request = Request::new().with_method("method".to_string())
                                 .with_route("route".to_string());

    assert_eq!(true, original != request)
}

#[test]
fn response_neq_test() {
    let original = Response::new();
    let request = Response::new().with_status(404_i32)
                                 .with_content_type(ContentType::TextHtml)
                                 .with_body(b"content".to_vec());

    assert_eq!(true, original != request)
}
<<<<<<< HEAD

#[test]
fn test_set_configs() {
    let host = String::from("testing");
    let port = String::from("the");
    let static_dir = String::from("configs");
    let html_dir = String::from("and");
    servo::set_host(&host);
    servo::set_port(&port);
    servo::set_static_directory(&static_dir);
    servo::set_html_directory(&html_dir);
    let configs = servo::get_configs();
    println!("VAL: {}", configs.get_host());
    assert_eq!(true, host == configs.get_host());
    assert_eq!(true, port == configs.get_port());
    assert_eq!(true, static_dir == configs.get_static_directory());
    assert_eq!(true, html_dir == configs.get_html_directory());
}

#[test]
fn test_add_route() {
    let route = String::from("my test route");
    servo::add_route(route.clone(), _testing_callback);
    let configs = servo::get_configs();
    assert!(configs.routes.route_map.contains_key(&route));
    let func = configs.routes.route_map.get(&route).expect("Invalid Response");
    assert!(func(Request::new()) == _testing_callback(Request::new()));
}

fn _testing_callback(req: Request) -> Response {
    servo::http::ok(String::from("test me"), CONTENT_TYPE::TEXT_HTML)
}

// Tests for parsing the query string to params and args
#[test]
fn test_params_from_route() {
    let mut request = Request::new().with_route("/test/route/param?arg1=1&arg2=2".to_string());
    let query: Vec<&str> = ["param", "arg1=1&arg2=2"].to_vec();
    request = request.params_from_route(query);
    assert_eq!(request.get_params(), ["param".to_string()].to_vec())
}

#[test]
fn test_multiple_params_from_route() {
    let mut request = Request::new()
                              .with_route("/test/route/param1?param2?arg1=1&arg2=2".to_string());
    let query: Vec<&str> = ["param1", "param2", "arg1=1&arg2=2"].to_vec();
    request = request.params_from_route(query);
    assert_eq!(request.get_params() , ["param1".to_string(), "param2".to_string()].to_vec())
}

#[test]
fn test_empty_params_from_route() {
    let mut request = Request::new()
                              .with_route("/test/route/arg1=1&arg2=2".to_string());
    let query: Vec<&str> = ["arg1=1&arg2=2"].to_vec();
    request = request.params_from_route(query);
    let compare: Vec<String> = Vec::new();
    assert_eq!(request.get_params() , compare)
}

#[test]
fn test_args_from_route() {
    let mut request = Request::new().with_route("/test/route/param?arg1=1".to_string());
    let mut query: Vec<&str> = ["param", "arg1=1"].to_vec();
    let args: HashMap<String,String> = [("arg1".to_string(), "1".to_string())]
                                        .iter().cloned().collect();
    request = request.args_from_route(query.as_mut());
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_multiple_args_from_route() {
    let mut request = Request::new().with_route("/test/route/param?arg1=1&arg2=2".to_string());
    let mut query: Vec<&str> = ["param", "arg1=1&arg2=2"].to_vec();
    let args: HashMap<String,String> = [("arg1".to_string(), "1".to_string()),
                                        ("arg2".to_string(), "2".to_string())]
                                        .iter().cloned().collect();
    request = request.args_from_route(query.as_mut());
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_empty_args_from_route() {
    let mut request = Request::new().with_route("/test/route/param?".to_string());
    let mut query: Vec<&str> = ["param"].to_vec();
    let args: HashMap<String,String> = HashMap::new();
    request = request.args_from_route(query.as_mut());
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_parse_query_string_single() {
    let mut request = Request::new().with_route("/test/route/param?arg1=1".to_string());
    let args: HashMap<String,String> = [("arg1".to_string(), "1".to_string())]
                                        .iter().cloned().collect();
    request = request.parse_query_string();
    assert_eq!(request.get_params() , ["param".to_string()].to_vec());
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_parse_query_string_multiple() {
    let mut request = Request::new()
                              .with_route("/test/route/param1?param2?arg1=1&arg2=2".to_string());
    let args: HashMap<String,String> = [("arg1".to_string(), "1".to_string()),
                                        ("arg2".to_string(), "2".to_string())]
                                        .iter().cloned().collect();
    request = request.parse_query_string();
    assert_eq!(request.get_params() , ["param1".to_string(), "param2".to_string()].to_vec());
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_parse_query_string_none() {
    let mut request = Request::new().with_route("/test/route/".to_string());
    let params: Vec<String> = Vec::new();
    let args: HashMap<String,String> = HashMap::new();
    request = request.parse_query_string();
    assert_eq!(request.get_params() , params);
    assert_eq!(request.get_args() , args)
}

#[test]
fn test_parse_query_string_wildcard() {
    let mut request = Request::new().with_route("/test/route/param?arg1=1/{}".to_string());
    let args: HashMap<String,String> = [("arg1".to_string(), "1".to_string())]
                                        .iter().cloned().collect();
    request = request.parse_query_string();
    assert_eq!(request.get_params() , ["param".to_string()].to_vec());
    assert_eq!(request.get_args() , args)
}
