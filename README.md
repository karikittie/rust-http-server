# Simple HTTP Server

## Required Files
These are files and/or directory that you must create in the root directory of the project to use the crate properly.

```
views.rs | contains views to route requests to
templates/ | directory that contains html files
```

## Configuring Views
Views that serve HTTP routes are configured in the views.rs file. In this file, there is a view config function<br>
`view_config() -> HashMap<String, CallBack>`<br>
In this function, you will add routes in the following way:
```
fn view_config() -> HashMap<String, CallBack> {
    let mut views = HashMap::new();

    views.insert("GET /".into(), my_func);
    views.insert("POST /login".into(), my_login_func);

    views
}
```

## Configuring Server Options
Server configuration options also have their own function. These options are in the server.rs file within the function:<br>
`server_config() -> HashMap<String, String>`<br>
However, you only need to edit the defaults that are already in place in this function like so:
```
fn server_config() -> HashMap<String, String> {
    let mut server = HashMap::new();

    server.insert("host".into(), "127.0.0.1".into());
    server.insert("port".into(), "8000".into());

    server
}
```
