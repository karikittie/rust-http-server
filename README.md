# Simple HTTP Server

## Required Files
These are files and/or directory that you must create in the root directory of the project to use the crate properly.

```
views.rs | contains views to route requests to
templates/ | directory that contains html files
```

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

## Configuring Server Options
Server configuration options can be configured using the methods:<br>
```
servo::set_port(String)
servo::set_host(String)
```
