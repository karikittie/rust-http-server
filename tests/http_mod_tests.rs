extern crate servo;
use servo::http::{Request, Response};
use servo::http::content_type::ContentType;
use std::collections::HashMap;


#[test]
// Checks that the equality trait works for Request objects
fn request_eq_test(){
    let request1 = Request::new();
    let request2 = Request::new();
    assert_eq!(true, request1 == request2)
}

#[test]
// Checks that the equality trait works for request objects
fn response_eq_test() {
    let response1 = Response::new();
    let response2 = Response::new();
    assert_eq!(true, response1 == response2)
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

// Tests for parsing the query string to params and args
#[test]
fn test_params_from_route() {
    let mut request = Request::new().with_route("/test/route/query?arg1=1&arg2=2".to_string());
    let params: HashMap<String, String> = [("arg1".to_string(), "1".to_string()),
                                           ("arg2".to_string(), "2".to_string())]
                                           .iter().cloned().collect();
    request = request.query_params_from_route();
    assert_eq!(request.get_query_params(), params)
}

#[test]
fn test_multiple_params_from_route() {
    let mut request = Request::new().with_route("/test/route/query1?query2?arg1=1&arg2=2".to_string());
    let params: HashMap<String, String> = [("arg1".to_string(), "1".to_string()),
                                           ("arg2".to_string(), "2".to_string())]
                                           .iter().cloned().collect();
    request = request.query_params_from_route();
    assert_eq!(request.get_query_params() , params)
}

#[test]
fn test_empty_params_from_route() {
    let mut request = Request::new()
                              .with_route("/test/route/".to_string());
    request = request.query_params_from_route();
    let params: HashMap<String, String> = HashMap::new();
    assert_eq!(request.get_query_params() , params)
}
