extern crate servo;
use servo::http::{Request, Response};
use servo::http::content_type::CONTENT_TYPE;
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
    let content = CONTENT_TYPE::TEXT_HTML;
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
                                 .with_content_type(CONTENT_TYPE::TEXT_HTML)
                                 .with_body(b"content".to_vec());

    assert_eq!(true, original != request)
}

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
