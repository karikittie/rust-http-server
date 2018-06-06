#![allow(dead_code)]

pub mod http;

use self::http::{Request, Response};
use self::http::content_type::{ContentType, get_content_type};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::net::{TcpStream, TcpListener};
use std::io::Write;
use std::str;
use std::thread;

/// Function type that all routes must map to.
/// Responses can be built using the built-in
/// functions in the servo::http module. These methods
/// take some form of HTTP body data and a content type
/// enum specifier (servo::http::CONTENT_TYPE).
pub type CallBack = fn(Request, &Configuration) -> Response;


/// Function type of the routing function for
/// Servo. This allows anybody to inject their own
/// routing algorithm that will override Servo's
/// default routing. It takes a
/// Request route and returns a vector of URL arguments
/// that will be attached to the Request given to the CallBack.
pub type Router = fn(&Request, &Routes) -> (Vec<String>, CallBack);

pub struct Servo {
    configuration : Configuration
}

// TODO: we need to add configs for the host and port it listens on.
//     we should also think about limiting the number of threads that will
//     be created to protect against DOS attacks.
/// Starts the server on port 8000 listening to localhost.
/// Then a stream is created and a new thread is spun up for every request.
///
impl Servo {
    pub fn new() -> Servo {
        Servo {
            configuration : Configuration::new()
        }
    }

    pub fn with_configuration(mut self, configuration: Configuration) -> Servo {
        self.configuration = configuration;
        self
    }

    pub fn run(&self) {
        let host = self.configuration.server.get_host();
        let port = self.configuration.server.get_port();
        println!("Starting server...");
        let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
        println!("Listening on {}:{}", host, port);

        for stream in listener.incoming() {
            let configs = self.configuration.clone();
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_request(stream, &configs)
                    });
                },
                Err(e) => eprintln!("Error in handling request: {}", e),
            }
        }
    }
}

// All server configs should exist here. Each with a corresponding constant key
pub struct Server {
    host : String,
    port : String,
    static_dir : String,
    html_dir : String,
    router : Router,
}

impl Server {
    pub fn new() -> Server {
        Server {
            host : String::from("127.0.0.1"),
            port : String::from("8000"),
            static_dir : String::from("static/"),
            html_dir : String::from("templates/"),
            router : default_router
        }
    }

    /// Returns the host in the format `123.123.123.123`
    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    /// Returns the port in the format `1234` (no colons)
    pub fn get_port(&self) -> String {
        self.port.clone()
    }

    pub fn get_static_directory(&self) -> String {
        self.static_dir.clone()
    }

    pub fn get_html_directory(&self) -> String {
        self.html_dir.clone()
    }

    pub fn with_host(mut self, host: &str) -> Server {
        self.host = String::from(host);
        self
    }

    pub fn with_port(mut self, port: &str) -> Server {
        self.port = String::from(port);
        self
    }

    pub fn with_static_dir(mut self, static_dir: &str) -> Server {
        self.static_dir = String::from(static_dir);
        self
    }

    pub fn with_html_dir(mut self, html_dir: &str) -> Server {
        self.html_dir = String::from(html_dir);
        self
    }

    pub fn with_router(mut self, router: Router) -> Server {
        self.router = router;
        self
    }

    pub fn route_request(&self, request: &Request, routes: &Routes) -> (Vec<String>, CallBack) {
        (self.router)(request, &routes)
    }

    pub fn clone(&self) -> Server {
        Server {
            host: self.host.clone(),
            port: self.port.clone(),
            static_dir: self.static_dir.clone(),
            html_dir: self.html_dir.clone(),
            router: self.router
        }
    }
}

/// Holds route configuration information. This shouldn't be needed
/// to run or maintain the server but the definition is provided publicly
/// for reference through the Configuration struct.
pub struct Routes {
    pub route_map: BTreeMap<String, CallBack>,
}

impl Routes {
    pub fn new() -> Routes {
        let mut map: BTreeMap<String, CallBack> = BTreeMap::new();
        map.insert(String::from("GET /"), default_home);
        Routes {
            route_map: map
        }
    }

    pub fn with_route(mut self, route: &str, callback: CallBack) -> Routes {
        self.route_map.insert(String::from(route), callback);
        self
    }

    pub fn contains_route(&self, route: &str) -> bool {
        self.route_map.contains_key(route)
    }

    pub fn get_route(&self, route: &str) -> Option<&CallBack> {
        self.route_map.get(route)
    }

    pub fn add_route(&mut self, key: &str, callback: CallBack) {
        self.route_map.insert(String::from(key), callback);
    }

    pub fn clone(&self) -> Routes {
        Routes {
            route_map: self.route_map.clone()
        }
    }
}

/// Holds server configuration information.
pub struct Configuration {
    pub server: Server,
    pub routes: Routes,
}

/// This struct contains all of the configuration setup variables
/// needed to start and run the server. Making changes to this object
/// WILL NOT update configurations. Any configuration changes need to be
/// done through the setter functions in the servo module. As such, this
/// struct cannot be built using `Configuration::new()` and this on purpose.
/// If you need this object, you can get it through `servo::get_configs()`.
impl Configuration {
    pub fn new() -> Configuration {
        let config = Configuration {
            server: Server::new(),
            routes: Routes::new(),
        };
        config
    }

    pub fn with_server_configurations(mut self, configs: Server) -> Configuration {
        self.server = configs;
        self
    }

    pub fn with_routes(mut self, routes: Routes) -> Configuration {
        self.routes = routes;
        self
    }

    pub fn clone(&self) -> Configuration {
        Configuration {
            server: self.server.clone(),
            routes: self.routes.clone()
        }
    }
}

/// This function serves static files based on the defined static directory.
/// The default static files directory is `static/`. Static files are served at
/// `/static/{file path under static directory}`
fn static_route(request: Request, config: &Configuration) -> Response {
    let file_to_get = request.get_url_args();
    let static_dir = config.server.get_static_directory();
    let filename = format!("{}{}", static_dir, file_to_get);
    let file_to_serve = File::open(&filename);
    match file_to_serve {
        Ok(mut file) => {
            let mut contents: Vec<u8> = Vec::new();
            let result = file.read_to_end(&mut contents);
            match result {
                Ok(_) => http::ok_file(contents, get_content_type(&filename)),
                Err(e) => {
                    eprintln!("File read error: {}", e);
                    http::not_found(String::from("Could not read file"), ContentType::TextHtml)
                },
            }
        },
        Err(e) => {
            eprintln!("Could not find file to serve: {:?}",e);
            http::not_found(String::from("Could not find resource"), ContentType::TextHtml)
        },
    }
}

/// Retrieves a file as a String from the directory setup to contain
/// HTML files. The default directory is `templates/`. This is meant to be
/// used in conjunction with the functions that build a result from a body string
/// and a content type.
pub fn get_html(path: &str, config: &Configuration) -> String {
    let html_dir = config.server.get_html_directory();
    let filename = format!("{}{}", html_dir, path);
    let file = File::open(filename);
    match file {
        Ok(mut f) => {
            let mut result_string: String = String::new();
            let result = f.read_to_string(&mut result_string);
            match result {
                Ok(_) => {
                    result_string
                },
                Err(e) => {
                    eprintln!("File read error: {}", e);
                    String::from("")
                },
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            String::from("")
        },
    }
}

/// This only returns the not found route and is used when no
/// route can be found for the given request.
fn default_callback(_: Request, _: &Configuration) -> Response {
    http::not_found(String::from("Route not found."), ContentType::TextHtml)
}

/// This returns the default homepage that only says: Good Job
fn default_home(_: Request, _: &Configuration) -> Response {
    http::ok(String::from("Good Job."), ContentType::TextHtml)
}

/// This router has some limitations. It only correctly parses wildcard characters
/// defined at the end of a route, such as: `GET /home/{}` but nowhere else.
/// A route map defined as
/// <br>
/// "GET /home/{}/static/{}" => my_func
/// "GET /home/{}" => my_other_func
/// <br>
/// will parse `GET /home/113/static/115`
/// as a Request to `my_other_func` with the args `["113", "stuff", "115"]`.
/// Custom routers can be injected, however.
fn default_router(request: &Request, routes: &Routes) -> (Vec<String>, CallBack) {
    let mut func: Option<&CallBack> = None;
    let mut args: Vec<String> = Vec::new();
    let requested_route = request.get_route();
    let requested_method = request.get_method();
    if routes.contains_route(&requested_route) {
        func = routes.get_route(&requested_route);
    } else {
        let mut segments = requested_route.split("/").skip(1).collect::<Vec<&str>>();
        let mut cont = true;
        let mut i = segments.len() as i32;
        while cont && i >= 0 {
            args.push( match segments.pop() {
                Some(string) => String::from(string),
                None => String::from(""),
            });
            let mut check_me = segments.clone().iter().map(|seg| format!("{}/", seg)).collect::<String>();
            check_me = format!("{} /{}{}", requested_method, check_me, "{}");
            if routes.contains_route(&check_me) {
                func = routes.get_route(&check_me);
                cont = false;
            }
            i -= 1;
        }
    }
    if args.len() > 1 {
        args.reverse();
    }
    match func {
        Some(f) => (args, *f),
        None => (Vec::new(), default_callback),
    }
}

fn get_request(method: &str, route: &str) -> Request {
    Request::new()
        .with_method(String::from(method))
        .with_route(String::from(route))
}

#[test]
fn test_route_default() {
    let mut configs = Configuration::new();
    configs.routes.add_route("GET /", |_, _| http::ok(String::from("found"), ContentType::TextHtml));
    let (_, callback) = default_router(&get_request("GET", "/"), &configs.routes);
    let resp = callback(Request::new(), &configs);
    assert_eq!(resp, http::ok(String::from("found"), ContentType::TextHtml));
}

#[test]
fn test_route_wildcard_simple() {
    let mut configs = Configuration::new();
    configs.routes.add_route("GET /{}", |_, _| http::ok(String::from("found"), ContentType::TextHtml));
    let (args, callback) = default_router(&get_request("GET", "/home"), &configs.routes);
    let resp = callback(Request::new(), &configs);
    assert_eq!(resp, http::ok(String::from("found"), ContentType::TextHtml));
    assert_eq!(args, vec![String::from("home")]);
}

#[test]
fn test_route_complex() {
    let mut configs = Configuration::new();
    configs.routes.add_route("GET /home/nope/ok", |_, _| http::ok(String::from("found"), ContentType::TextHtml));
    let (_, callback) = default_router(&get_request("GET", "/home/nope/ok"), &configs.routes);
    let resp = callback(Request::new(), &configs);
    assert_eq!(resp, http::ok(String::from("found"), ContentType::TextHtml));
}

#[test]
fn test_route_wildcard_complex() {
    let mut configs = Configuration::new();
    configs.routes.add_route("GET /home/nope/{}", |_, _| http::ok(String::from("found"), ContentType::TextHtml));
    let (args, callback) = default_router(&get_request("GET", "/home/nope/blah/whatever/113"), &configs.routes);
    let resp = callback(Request::new(), &configs);
    assert_eq!(resp, http::ok(String::from("found"), ContentType::TextHtml));
    assert_eq!(args, vec![String::from("blah"), String::from("whatever"), String::from("113")]);
}

pub fn route_request(request: Request, configs: &Configuration) -> Response {
    let (args, callback) = configs.server.route_request(&request, &configs.routes);
    callback(request.with_url_args(args), configs)
}

// TODO: we might want to return a Result<Vec<u8>> to push the error handling downstream
/// Takes a TCP buffer, reads whatever is in it and outputs
/// the contents to a u8 Vector. If there is an error reading
/// the buffer, an error is printed to console and an empty
/// vector is returned.
fn read_input_buffer(mut stream : &TcpStream) -> Vec<u8> {
    let mut buffer = [0u8 ; 4096];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            Vec::from(request.as_bytes())
        },
        Err(e) => {
            eprintln!("Input Stream Error: {}", e);
            vec![]
        },
    }
}

// TODO: We also might want to push error handling out here. I'd like to use some actual logging here instead of printing to console.
/// Takes a u8 array and the TCP stream and writes those bytes to the stream.
/// Prints 'replied' on successful write and an error message on failure.
fn write_output_buffer(mut stream : TcpStream, to_write : &[u8]) {
    match stream.write(to_write) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to reply to request: {}", e),
    }
}

// TODO: http_requests should be transformed into a module.
/// Reads from the input buffer and transforms that Vec<u8> into a String.
/// Then it transforms that String into a Request object and asks for the
/// Response String from the http_requests file functions and writes that output
/// to the stream.
fn handle_request(stream : TcpStream, configs: &Configuration) {
    let vector_buffer = read_input_buffer(&stream);
    let request_str =
        String::from(match str::from_utf8(&vector_buffer) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("Error: {}", err);
                ""
            },
        });
    let request_obj = http::Request::from(&request_str);
    let mut response = route_request(request_obj, configs);
    let response_bytes = response.byteify();
    write_output_buffer(stream, &response_bytes);
}
