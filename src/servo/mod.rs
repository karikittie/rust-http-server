pub mod http;

use self::http::{Request, Response};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::open;

pub type CallBack = fn(Request) -> Response;

// Public constants
pub const HOST: &'static str = "host";
pub const PORT: &'static str = "port";
pub const STATIC_DIR: &'static str = "sdir";

// Static vars are to avoid having to do dependency injection to have
// configuration persistence.
static mut CONFIGS: Option<Configuration> = None;
static mut SERVER_CONFIGS: Option<HashMap<&'static str, String>> = None;
static mut ROUTE_CONFIGS: Option<HashMap<String, CallBack>> = None;

// All server configs should exist here. Each with a corresponding constant key
pub struct Server {
    host: String,
    port: String,
    static_dir: String,
}

// Route to function mappings
pub struct Routes {
    pub route_map: &'static HashMap<String, CallBack>,
}

// Holds all program configuration variables
pub struct Configuration {
    pub server: Server,
    pub routes: Routes,
}

/* 
This aids the user in getting config variables. User should never create a new Configuration object.
Has getters for all non-route configurations. The 'routes' HashMap is just cloned and given back
to the user.
*/
impl Configuration {
    fn new() -> Configuration {
        let server_conf = server_config();
        let route_conf = route_config();
        let config = Configuration{
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
                                    .clone()
                            },
            routes: Routes {route_map: route_conf}
        };
        config
    }

    pub fn get_host(&self) -> String {
        self.server.host.clone()
    }

    pub fn get_port(&self) -> String {
        self.server.port.clone()
    }
}

// Adds a route to the static route table.
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

// Allows the user to set the host the server will listen on
pub fn set_host(host: &str) {
    set_server_attr(HOST, String::from(host));
}

// Allows the user to set the port the server will listen on
pub fn set_port(port: &str) {
    set_server_attr(PORT, String::from(port));
}

// Allows the user to set the static file directory the server will look in
pub fn set_static_directory(dir: &str) {
    set_server_attr(STATIC_DIR, String::from(dir));
}

/*
This is a private method used to set a general server attribute to the
static server HashMap.
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

// This is how the user will get the app configs
pub fn get_configs<'a>() -> &'a Configuration {
    unsafe {
        match &mut CONFIGS {
            &mut Some(ref config) => &config,
            &mut None => {
                let new_configs = Configuration::new();
                CONFIGS = Option::from(new_configs);
                get_configs()
            },
        }
    }
}

// This is how this module gets the server configs internally.
// It sets defaults if they aren't already set.
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
                    conf.insert(STATIC_DIR, String::from("./static/"));
                }
            },
            &mut None => {
                let mut config: HashMap<&'static str, String> = HashMap::new();
                config.insert(HOST, String::from("127.0.0.1"));
                config.insert(PORT, String::from("8000"));
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

// Gets the route map. Should return a default route if one is not already entered for
// 'GET /'.
fn route_config() -> &'static HashMap<String, CallBack> {
    unsafe {
        match &mut ROUTE_CONFIGS {
            &mut Some(ref conf) => {
                conf
            },
            &mut None => {
                let conf: HashMap<String, CallBack> = HashMap::new();
                //conf.insert(String::from("GET /"), default_home); TODO: add this view
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
    let file_to_get = String::from(format!("../{}/", static_dir));
    let static_url_len = "/static/".len();
    file_to_get.push_str(&request.get_route().chars().skip(static_url_len).collect::<String>());
    let filename = format!("{}{}", static_dir, file_to_get);
    let file_to_serve = File::open(filename);
    match file_to_serve {
        Ok(x) => {

        },
        Err(e) => {
            println!("Could not find file to serve: {:?}",e);
        },
    }
    http::ok(String::from("stuff")) // TODO: implement this!
}

pub fn route_request(request: Request) -> Response {
    let config = route_config();
    let func = config.get(&request.get_route());
    match func {
        Some(f) => {
            f(request)
        },
        None => {
            http::bad_route()
        },
    }
}
