# Simple HTTP Server

## Required Files
These are files and/or directory that you must create in the root directory of the project to use the crate properly.

```
.config | configuration file
views.rs | contains views to route requests to
templates/ | directory that contains html files
```

## Config File
The config file is named '.config' and is located in the root directory. Routes are handled by naming a method, route string and a view to route to. The view corresponds to a function in the 'views.rs' file that takes a Request object and returns a String body. There is an example of a config file below:

[routes]<br>
GET / home-view `<-- This is a default route`<br>
GET /some-subpage subpage-view<br>
POST /login login-view

[server]<br>
host: 127.0.0.1 `<-- This is the default`<br>
port: 8000 `<-- This is the default`
