# Servo, the simple HTTP server

## Basic Setup
Servo is designed to be simple to use! To get started, you need only include the crate and put the following lines of code in your main.rs file:
```
    extern crate servo;
    use servo::Servo;

    fn main() {
        let servo = Servo::new();
        servo.run();
    }
```
This will start a server on `localhost:8000/` that returns a simple message. The default server will serve static files from `static/` (relative to the root directory) and serve HTML files from `templates/`. Everything from here on out is configurable. You can do this by adding your own functions that make use of Servo's libraries to set configuration variables. The rest of this document will explain how to correctly set these configurations to get the most out of Servo, the simple HTTP server!

## Configuring Views
Views are configured using the method<br>
`my_configs.routes.add_route(String, CallBack)`<br>
or<br>
`Routes::new().with_route(String, CallBack)`<br>
where:<br>
`type CallBack = fn(Request, &Configuration) -> Response`<br>
Views take a servo::http::Request and return a servo::http::Response. There are several helper methods for this:
```
servo::http::ok(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 200
servo::http::ok_file(body: Vec<u8>, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 200
servo::http::not_found(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 404
servo::http::server_error(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 505
```
You may also construct a Response from scratch using the builder pattern like so:
```
Response::new()
    .with_status(200)
    .with_content_type(ContentType::TextHtml)
    .with_body(Vec::from(body_string.as_bytes()));
```
There is also a method called `get_html(&str, &Configuration) -> String` which will pull from the HTML directory defined in the configs. 
The default directory for these is `templates/`. This can be used with the other functions to return HTML like so:<br>
`ok(get_html("my_file.html", configs), CONTENT_TYPE::TEXT_HTML))`

## File Serving
Static files can be served in a couple of ways. Firstly, you can setup a directory where your static files will be served from. This is relative to your root directory and is set to `static/` by default. They are served from this directory to the URL `/static/` by default. There isn't currently a way to define a different static files URL but the directory may be set by calling `my_configs.server.set_static_dir(new_directory)`. You are able to overwrite this route by setting a new route using
<br>
`my_routes.add_route("GET /static/{}", some other callback function))`<br>
You should be cautious not to do this since it will break static file serving.<br>
You can also setup a directory that Servo will pull HTML files from using the function<br>
`servo::get_html(&str, &Configuration) -> String`<br>
The function to set this option is `my_configs.server.with_html_dir(new directory)`.

## Configuration Object
Creating a new instance of a Configuration struct via `servo::Configuration::new()` gives you a struct with your server configurations (including all the defaults already filled in)! Here's the structure:
```
Configuration {
    Server {
        host: "127.0.0.1",
        port: "8000",
        domain: "localhost",
        static directory: "static/",
        html directory: "templates/",
        routing function: Servo's default routing
    },
    Routes {
        route map {
            "GET /" => Servo's default homepage,
            "GET /static/{}" => Servo's static file serving
        }
    }
}
```
All structs can be built using the pattern:
```
let struct = Struct::new()
    .with_attribute(my_custom_attribute)
    .with_other_attr(another_custom_attr);
```
You'll notice the Server also allows you to inject your own routing function into Servo. Servo's internal routing function is relatively simple so, if you'd like more complex capability, you can create your own function with the signature:
```
fn(&Request, &Routes) -> (Vec<String>, CallBack)
```
where the Vec<String> is a list of URL arguments internal to the route and apart from the URL query parameters. This can then be set using `my_server.with_router(my_router)`.

## Sample main.rs
```
extern crate servo;

use servo::{Servo, Configuration, Server, Routes};
use servo::get_html;
use servo::http::{Request, Response, ok};
use servo::http::content_type::ContentType;

fn main() {
    // Redundantly setting to default configs to show how
    let mut configs = Configuration::new()
        .with_server_configurations(
            Server::new()
                .with_host("127.0.0.1")
                .with_port("8000")
                .with_domain("localhost")
                .with_static_dir("static/")
                .with_html_dir("templates/")
        )
        .with_routes(
            // These routes are not default
            Routes::new()
                .with_route("GET /", my_new_home)
                .with_route("GET /home", my_new_home)
        );
    configs.routes.add_get("/home/{}", wildcard_route);
    let servo = Servo::new().with_configuration(configs);
    servo.run();
}

// This is an example CallBack function
fn my_new_home(_: Request, configs: &Configuration) -> Response {
    ok(get_html("index.html", configs), ContentType::TextHtml)
}

// This is another example CallBack function that serves an image
fn wildcard_route(_: Request, configs: &Configuration) -> Response {
    ok(format!("<html><img src=\"{}my_file.jpg\" ></html>", configs.get_static_uri()), ContentType::TextHtml)
}
```