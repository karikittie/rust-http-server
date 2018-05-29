# Servo, the simple HTTP server

## Basic Setup
Servo is designed to be simple to use! To get started, you need only include the crate and run the server binary manually or by using `cargo run`. This will start a server on `localhost:8000/` that returns a simple message. The default server will serve static files from `static/` (relative to the root directory) and serve HTML files from `templates/`. Everything from here on out is configurable. You can do this by adding your own functions that make use of Servo's libraries to set configuration variables. The rest of this document will explain how to correctly set these configurations to get the most out of Servo, the simple HTTP server!

## Configuring Views
Views are configured using the method<br>
`servo::add_route(String, CallBack)`<br>
where:<br>
`type CallBack = fn(Request) -> Response`<br>
Views take a servo::http::Request and return a servo::http::Response. There are several helper methods for this:
```
servo::http::ok(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 200
servo::http::ok_file(body: Vec<u8>, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 200
servo::http::not_found(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 404
servo::http::server_error(body: String, content_type: servo::http::content_type::CONTENT_TYPE) // Status = 505
```
There is also a method called `get_html(filename) -> String` which will pull from the HTML directory defined in the configs. 
The default directory for these is `templates/`. This can be used with the other functions to return HTML like so:<br>
`return ok(get_html("my_file.html", CONTENT_TYPE::TEXT_HTML))`

## File Serving
Static files can be served in a couple of ways. Firstly, you can setup a directory where your static files will be served from. This is relative to your root directory and is set to `static/` by default. They are served from this directory to the URL `/static/` by default. There isn't currently a way to define a different static files URL but the directory may be set by calling `servo::set_static_dir(new_directory)`. You are able to overwrite this route by setting a new route using
<br>
`servo::add_route(String::new("/static/{}", some other callback function))`<br>
You should be cautious not to do this since it will break static file serving.<br>
You can also setup a directory that Servo will pull HTML files from using the function<br>
`servo::get_html(filename)`<br>
The function to set this option is `servo::set_html_dir(new directory)`.

## Configuring Server Options
Server configuration options can be configured using the methods:<br>
```
servo::set_port(String)
servo::set_host(String)
```
