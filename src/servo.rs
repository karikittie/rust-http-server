mod http;
mod views;

pub mod servo {
    use std::collections::HashMap;
    use http::http::{Request, Response};

    type CallBack = fn(Request) -> Response;

    const HOST: &'static str = "host";
    const PORT: &'static str = "port";

    pub static configs: Option<Configuration> = None;

    pub struct Server {
        host: String,
        port: String,
    }

    pub struct Routes {
        route_map: HashMap<String, CallBack>,
    }

    pub struct Configuration {
        server: Server,
        routes: Routes,
    }

    impl Configuration {
        fn new() -> Configuration {
            let server_configs = server_config();
            let route_configs = route_config();
            let config = Configuration{
                server: Server {host: server_configs.get(HOST).expect("No host defined").clone(), 
                                port: server_configs.get(PORT).expect("No port defined").clone()},
                routes: Routes {route_map: route_configs}
            };
            config
        }
    }

    fn server_config() -> HashMap<&'static str, String> {
        let mut config: HashMap<&'static str, String> = HashMap::new();

        config.insert(HOST, "127.0.0.1".into());
        config.insert(PORT, "8000".into());

        config
    }

    fn route_config() -> HashMap<String, CallBack> {
        let mut config = HashMap::new();

        config
    }
}