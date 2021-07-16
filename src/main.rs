//! This project implements an multithreaded web server. It has the following functions:
//! 1. Listen for TCP connections on a socket.
//! 2. Parse a small number of HTTP requests.
//! 3. Create a proper HTTP response.
//! Achieve a good throughput with a thread pool
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {

}