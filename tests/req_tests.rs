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
