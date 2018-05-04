use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::str;
use std::thread;

mod http_requests;
use http_requests::get_http_response;

fn read_input_buffer(mut stream : &TcpStream) -> Vec<u8> {
    let mut buffer = [0u8 ; 4096];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            println!("Handling request:\r\n{}", request);
            let req = request.into_owned();
            Vec::from(get_http_response(&req).as_bytes())
        },
        Err(e) => {
            println!("Input Stream Error: {}", e);
            vec![]
        },
    }
}

fn write_output_buffer(mut stream : TcpStream, to_write : &[u8]) {
    match stream.write(to_write) {
        Ok(_) => println!("Replied"),
        Err(e) => println!("Failed to reply to request: {}", e),
    }
}

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
    let response_str = get_http_response(&request_str);
    write_output_buffer(stream, response_str.as_bytes());
}

fn main() {
    println!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening on port {}", "8000");

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
