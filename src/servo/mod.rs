pub mod http;

use self::http::{Request, Response};
use self::http::content_type::{CONTENT_TYPE, get_content_type};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// Function type that all routes must map to.
/// Responses can be built using the built-in
/// functions in the servo::http module. These methods
/// take some form of HTTP body data and a content type
/// enum specifier (servo::http::CONTENT_TYPE).
pub type CallBack = fn(Request) -> Response;

/// These constants are used for configuration setup.
/// They should not need to be used by client programs for
/// setup but are provided publicly for reference.
pub const HOST: &'static str = "host";
pub const PORT: &'static str = "port";
pub const STATIC_DIR: &'static str = "sdir";
pub const HTML_DIR: &'static str = "htmldir";

// Static vars are to avoid having to do dependency injection to have
// configuration persistence.
static mut SERVER_CONFIGS: Option<HashMap<&'static str, String>> = None;
static mut ROUTE_CONFIGS: Option<HashMap<String, CallBack>> = None;

// All server configs should exist here. Each with a corresponding constant key
pub struct Server {
    host: String,
    port: String,
    static_dir: String,
    html_dir: String,
}

/// Holds route configuration information. This shouldn't be needed
/// to run or maintain the server but the definition is provided publicly
/// for reference through the Configuration struct.
pub struct Routes {
    pub route_map: &'static HashMap<String, CallBack>,
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
    fn new() -> Configuration {
        let server_conf = server_config();
        let route_conf = route_config();
        let config = Configuration {
            server: Server {host: server_conf
                                    .get(HOST)
                                    .expect("No host defined")
                                    .clone(), 
                            port: server_conf
                                    .get(PORT)
                                    .expect("No port defined")
                                    .clone(),
                            static_dir: server_conf
                                    .get(STATIC_DIR)
                                    .expect("No static directory defined")
                                    .clone(),
                            html_dir: server_conf
                                    .get(HTML_DIR)
                                    .expect("No html directory defined")
                                    .clone()
                            },
            routes: Routes {route_map: route_conf},
        };
        config
    }

    /// Returns the host in the format `123.123.123.123`
    pub fn get_host(&self) -> String {
        self.server.host.clone()
    }

    /// Returns the port in the format `1234` (no colons)
    pub fn get_port(&self) -> String {
        self.server.port.clone()
    }

    pub fn get_static_directory(&self) -> String {
        self.server.static_dir.clone()
    }

    pub fn get_html_directory(&self) -> String {
        self.server.html_dir.clone()
    }
}

/// Sets up a route that will map to a CallBack function that
/// takes a Request and returns a Response. Information on how
/// to correctly setup routes can be found in the README.
pub fn add_route(route_string: String, callback: CallBack) {
    unsafe {
        match &mut ROUTE_CONFIGS {
            &mut Some(ref mut conf) => {
                conf.insert(route_string, callback);
            },
            &mut None => {
                let conf = HashMap::new();
                ROUTE_CONFIGS = Option::from(conf);
                add_route(route_string, callback);
            }
        }
    }
}

/// Sets the host IP that the server will listen on.
/// Don't set host as `127.0.0.1:8000`, you should instead
/// set it as `127.0.0.1` and call `servo::set_port` to
/// set the port.
pub fn set_host(host: &str) {
    set_server_attr(HOST, String::from(host));
}

/// Sets the port that the server will listen on
pub fn set_port(port: &str) {
    set_server_attr(PORT, String::from(port));
}

/// Sets the static directory that the static files route
/// will pull files from.
pub fn set_static_directory(dir: &str) {
    set_server_attr(STATIC_DIR, String::from(dir));
}

/// Sets the HTML template directory that can be automatically
/// pulled from using the `get_html(filename)` function.
pub fn set_html_directory(dir: &str) {
    set_server_attr(HTML_DIR, String::from(dir));
}

/*
This is used to set a general server attribute to the
static server HashMap. All public setters should use this
function, if possible.
*/
fn set_server_attr(key: &'static str, value: String) {
    unsafe {
        match &mut SERVER_CONFIGS {
            &mut Some(ref mut conf) => {
                conf.insert(key, value);
            },
            &mut None => {
                let mut conf: HashMap<&'static str, String> = HashMap::new();
                conf.insert(key, value);
                SERVER_CONFIGS = Option::from(conf);
            }
        }
    }
}

/// Returns the Configuration object for the project.
/// Changes to this object do not affect the project's
/// configuration. This must be done using the setters
/// in this module.
pub fn get_configs() -> Configuration {
    Configuration::new()
}

/*
Get the static server configs and adds defaults (if not already set).
It does not alter the static configs, it creates a clone of them and
returns that. However, defaults are added to the static configs before
it is cloned.
*/
fn server_config() -> HashMap<&'static str, String> {
    unsafe {
        match &mut SERVER_CONFIGS {
            &mut Some(ref mut conf) => {
                if !conf.contains_key(HOST) {
                    conf.insert(HOST, String::from("127.0.0.1"));
                }
                if !conf.contains_key(PORT) {
                    conf.insert(PORT, String::from("8000"));
                }
                if !conf.contains_key(STATIC_DIR) {
                    conf.insert(STATIC_DIR, String::from("static/"));
                }
                if !conf.contains_key(HTML_DIR) {
                    conf.insert(HTML_DIR, String::from("templates/"));
                }
            },
            &mut None => {
                let mut config: HashMap<&'static str, String> = HashMap::new();
                config.insert(HOST, String::from("127.0.0.1"));
                config.insert(PORT, String::from("8000"));
                config.insert(STATIC_DIR, String::from("static/"));
                config.insert(HTML_DIR, String::from("templates/"));
                SERVER_CONFIGS = Option::from(config);
            },
        }
        clone_server_configs()
    }
}

// Clones the server configs. Used internally to avoid manipulating a static HashMap.
fn clone_server_configs() -> HashMap<&'static str, String> {
    let mut config: HashMap<&'static str, String> = HashMap::new();
    unsafe {
        match &SERVER_CONFIGS {
            &Some(ref conf) => {
                for key in conf.keys() {
                    config.insert(key, conf[key].clone());
                }
            },
            &None => (),
        }
    }
    config
}

// Gets the route map. Should return a default route if one is not already entered for 'GET /'.
fn route_config() -> &'static HashMap<String, CallBack> {
    unsafe {
        match &mut ROUTE_CONFIGS {
            &mut Some(ref conf) => {
                conf
            },
            &mut None => {
                let mut conf: HashMap<String, CallBack> = HashMap::new();
                conf.insert(String::from("GET /static/"), static_route);
                ROUTE_CONFIGS = Option::from(conf);
                route_config()
            },
        }
    }
}

/// This function serves static files based on the defined static directory.
/// The default static files directory is `static/`. Static files are served at
/// `/static/{file path under static directory}`
fn static_route(request: Request) -> Response {
    let config = server_config();
    let static_dir = config.get(STATIC_DIR).expect("Static directory not configured properly");
    let mut file_to_get = String::from("main.css"); // TODO: replace this with Request's arg[0]
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
                    http::not_found(String::from("Could not read file"), CONTENT_TYPE::TEXT_HTML)
                },
            }
        },
        Err(e) => {
            eprintln!("Could not find file to serve: {:?}",e);
            http::not_found(String::from("Could not find resource"), CONTENT_TYPE::TEXT_HTML)
        },
    }
}

/// Retrieves a file as a String from the directory setup to contain
/// HTML files. The default directory is `templates/`. This is meant to be
/// used in conjunction with the functions that build a result from a body string
/// and a content type.
pub fn get_html(path: &str) -> String {
    let config = server_config();
    let html_dir = config.get(HTML_DIR).expect("Static directory not configured properly");
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

pub fn route_request(request: Request) -> Response {
    let config = route_config();
    let func = config.get(&request.get_route());
    match func {
        Some(f) => {
            f(request)
        },
        None => {
            http::not_found(String::from(""), CONTENT_TYPE::TEXT_HTML)
        },
    }
}
