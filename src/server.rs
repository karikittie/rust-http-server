use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::str;
use std::thread;
use servo::get_configs;

mod servo;

// TODO: we might want to return a Result<Vec<u8>> to push the error handling downstream
/// Takes a TCP buffer, reads whatever is in it and outputs
/// the contents to a u8 Vector. If there is an error reading
/// the buffer, an error is printed to console and an empty
/// vector is returned.
fn read_input_buffer(mut stream : &TcpStream) -> Vec<u8> {
    let mut buffer = [0u8 ; 4096];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            Vec::from(request.as_bytes())
        },
        Err(e) => {
            println!("Input Stream Error: {}", e);
            vec![]
        },
    }
}

// TODO: We also might want to push error handling out here. I'd like to use some actual logging here instead of printing to console.
/// Takes a u8 array and the TCP stream and writes those bytes to the stream.
/// Prints 'replied' on successful write and an error message on failure.
fn write_output_buffer(mut stream : TcpStream, to_write : &[u8]) {
    match stream.write(to_write) {
        Ok(_) => (),
        Err(e) => println!("Failed to reply to request: {}", e),
    }
}

// TODO: http_requests should be transformed into a module.
/// Reads from the input buffer and transforms that Vec<u8> into a String.
/// Then it transforms that String into a Request object and asks for the
/// Response String from the http_requests file functions and writes that output
/// to the stream.
fn handle_request(stream : TcpStream) {
    let vector_buffer = read_input_buffer(&stream);
    let request_str = 
        String::from(match str::from_utf8(&vector_buffer) {
            Ok(x) => x,
            Err(err) => {
                print!("Error: {}", err);
                ""
            },
        });
    let request_obj = servo::http::Request::from(&request_str);
    let mut response = servo::route_request(request_obj);
    let response_bytes = response.byteify();
    write_output_buffer(stream, &response_bytes);
}

// TODO: we need to add configs for the host and port it listens on.
//     we should also think about limiting the number of threads that will
//     be created to protect against DOS attacks.
/// Starts the server on port 8000 listening to localhost.
/// Then a stream is created and a new thread is spun up for every request.
fn main() {
    println!("Starting server...");
    let configs = get_configs();
    let host = configs.get_host();
    let port = configs.get_port();
    servo::add_route(String::from("GET /"), default_home);
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    println!("Listening on {}:{}", host, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_request(stream)
                });
            },
            Err(e) => println!("Error in handling request: {}", e),
        }
    }
}

fn default_home(request: servo::http::Request) -> servo::http::Response {
    servo::http::ok(String::from("Good Job."), servo::http::content_type::CONTENT_TYPE::TEXT_HTML)
}
