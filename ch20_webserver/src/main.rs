use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listenner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listenner.incoming() {
        let stream = stream.unwrap();

        // println!("Connection Established");
        handle_connection(stream);
    }
}

// The stream parameter is mutable.
fn handle_connection(mut stream: TcpStream) {
    // 512 bytes shoul be big enough to hold the data of a basic request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response: String = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // The lossy part indicates that if the function sees an
    // invalid UTF-8 sequence, it will replace the invalid sequence
    // with �
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
