mod servo {
    use std::collections::HashMap;
    type CallBack = fn(Request) -> Response;

    const HOST: String = "host".into();
    const PORT: String = "port".into();

    pub static configs: Configuration = Configuration::new();

    struct Server {
        host: String,
        port: String,
    }

    struct Routes {
        route_map: HashMap<String, CallBack>,
    }

    struct Configuration {
        server: Server,
        routes: Routes,
    }

    impl Configuration {
        fn new() -> Configuration {
            let server_configs = server_config();
            let route_configs = route_config();         
            let config = Configuration{
                server: Server {host: server_configs[&HOST], port: server_configs[&PORT]},
                routes: Routes {route_map: route_configs}
            };
            config
        }
    }

    fn server_config() -> HashMap<String, String> {
        let mut config = HashMap::new();

        config.insert(HOST, "127.0.0.1".into());
        config.insert(PORT, "8000".into());

        config
    }

    fn route_config() -> HashMap<String, CallBack> {
        let mut config = HashMap::new();

        config.insert("GET /".into(), default_home);

        config
    }

    fn default_home(request: Request) -> Response {
        Response::new()
    }
}