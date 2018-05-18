extern crate servo;
use servo::requests::Request;
use servo::requests::Response;
use std::collections::HashMap;


#[test]
fn request_clone_test(){
    let method = "method".to_string();
    let route = "route".to_string();
    let mut map = HashMap::new();
    map.insert("test".to_string(), "header".to_string());

    let original = Request { method: method.clone(), route: route.clone(), headers: map.clone() };
    let mut request = Request::new().with_method(method).with_route(route).with_headers(map);
    assert_eq!(true, original == request)
}

#[test]
fn response_clone_test() {
    let status = 0_i32;
    let content = "content".to_string();
    let body = "body".to_string();
    let mut map = HashMap::new();
    map.insert("test".to_string(), "header".to_string());

    let original = Response { status: status,
			      content_type: content.clone(),
			      body: body.clone(),
			      headers: Option::from(map.clone()) };
    let mut response = Response::new().with_status(status)
				      .with_content_type(content)
				      .with_body(body)
				      .with_headers(Option::from(map));
    assert_eq!(true, original == response)
}
