//! This project implements an multithreaded web server. It has the following functions:
//! 1. Listen for TCP connections on a socket.
//! 2. Parse a small number of HTTP requests.
//! 3. Create a proper HTTP response.
//! Achieve a good throughput with a thread pool
use std::io::prelude::*;// read write seek
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/// HTTP Request format:
///
/// Method Request-URI HTTP-Version CRLF
///
/// headers CRLF
///
/// message-body
///
/// HTTP Response format:
///
/// HTTP-Version Status-Code Reason-Phrase CRLF
///
/// headers CRLF
///
/// message-body
///
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]).split(' ').next().unwrap()); // lossy means when it sees an invalid UTF-8 sequence,

    let get = b"GET / HTTP/1.1\r\n"; // byte string
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("static/hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Length:{}\r\n\r\n\
        {}",
            contents.len(),
            contents
        );


        // it will replace it with 'U+FFFD'
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else { // response 404
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("static/404.html").unwrap();

        let response = format!(
            "{}\r\n\
            Content-Length: {}\r\n\r\n\
            {}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let mut lines = raw_request.lines();
    let first_line = lines.next().unwrap();
    let mut first_line_split = first_line.split(' ');
    let req_type = first_line_split.next().unwrap();
    let req_url = first_line_split.next().unwrap();
    let req_ver = first_line_split.next().unwrap();
    match req_type {
        "GET" => {
            println!("GET {}", req_url);
        },
        s => println!("other {} {}", s, req_url),
    }
}