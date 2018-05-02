use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

mod http_requests;
use http_requests::get_http_response;

const OUTPUT_BUFFER_SIZE: usize = 4096;

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
    write_output_buffer(stream, b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n");
}

// fn partition_byte_vector(output_vector: &Vec<u8>) -> Vec<[u8 ; OUTPUT_BUFFER_SIZE]> {
//     let parition_num: u32 = (output_vector.len() as f32 / OUTPUT_BUFFER_SIZE as f32).ceil() as u32;
//     let resp_vec: Vec<[u8 ; OUTPUT_BUFFER_SIZE]> = vec![];
//     for i in 0..parition_num {
//         let mut arr: [u8 ; OUTPUT_BUFFER_SIZE];
//         let start = i * OUTPUT_BUFFER_SIZE as u32;
//         for j in 0..OUTPUT_BUFFER_SIZE as u32 {
//             arr[j] = output_vector[(j + start) as usize];
//         }
//     }
//     resp_vec
// }

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
