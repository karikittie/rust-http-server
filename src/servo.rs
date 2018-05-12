pub mod servo {
    use std::collections::HashMap;
    use http::http::{Request, Response};

    type CallBack = fn(Request) -> Response;

    const HOST: &'static str = "host";
    const PORT: &'static str = "port";

    static mut configs: Option<Configuration> = None;
    static mut server_configs: Option<HashMap<&'static str, String>> = None;
    static mut route_configs: Option<HashMap<String, CallBack>> = None;

    pub struct Server {
        pub host: String,
        pub port: String,
    }

    pub struct Routes {
        pub route_map: &'static HashMap<String, CallBack>,
    }

    pub struct Configuration {
        pub server: Server,
        pub routes: Routes,
    }

    impl Configuration {
        fn new() -> Configuration {
            let server_conf = server_config();
            let route_conf = route_config();
            let config = Configuration{
                server: Server {host: server_conf.get(HOST).expect("No host defined").clone(), 
                                port: server_conf.get(PORT).expect("No port defined").clone()},
                routes: Routes {route_map: route_conf}
            };
            config
        }
    }

    pub fn add_route(route_string: String, callback: CallBack) {
        unsafe {
            match &mut route_configs {
                &mut Some(ref mut conf) => {
                    conf.insert(route_string, callback);
                },
                &mut None => {
                    let conf = HashMap::new();
                    route_configs = Option::from(conf);
                    add_route(route_string, callback);
                }
            }
        }
    }

    pub fn set_host(host: String) {
        set_server_attr(HOST, host);
    }

    pub fn set_port(port: String) {
        set_server_attr(PORT, port);
    }

    fn set_server_attr(key: &'static str, value: String) {
        match &mut server_configs {
            &mut Some(ref mut conf) => {
                conf.insert(key, value);
            },
            &mut None => {
                let mut conf: HashMap<&'static str, String> = HashMap::new();
                conf.insert(key, value);
                server_configs = Option::from(conf);
            }
        }
    }

    pub fn get_configs<'a>() -> &'a Configuration {
        unsafe {
            match &mut configs {
                &mut Some(ref config) => &config,
                &mut None => {
                    let new_configs = Configuration::new();
                    configs = Option::from(new_configs);
                    get_configs()
                },
            }
        }
    }

    fn server_config() -> HashMap<&'static str, String> {
        match &mut server_configs {
            &mut Some(ref conf) => {
                if !conf.contains_key(HOST) {
                    conf.insert(HOST, String::from("127.0.0.1"));
                }
                if !conf.contains_key(PORT) {
                    conf.insert(PORT, String::from("8000"));
                }
            },
            &mut None => {
                let mut config: HashMap<&'static str, String> = HashMap::new();
                config.insert(HOST, String::from("127.0.0.1"));
                config.insert(PORT, String::from("8000"));
                server_configs = Option::from(config);
            },
        }
        server_configs.expect("Server incorrectly configured")
    }

    fn route_config() -> &'static HashMap<String, CallBack> {
        unsafe {
            match &mut route_configs {
                &mut Some(ref conf) => {
                    conf
                },
                &mut None => {
                    let conf: HashMap<String, CallBack> = HashMap::new();
                    conf.insert(String::from("GET /"), default_home);
                    route_configs = Option::from(conf);
                    route_config()
                },
            }
        }
    }

    fn default_home(request: Request) -> Response {
        
    }
}