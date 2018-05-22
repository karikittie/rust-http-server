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

## Configuring Server Options
Server configuration options can be configured using the methods:<br>
```
servo::set_port(String)
servo::set_host(String)
```
