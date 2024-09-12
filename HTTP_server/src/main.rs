
use std::{
    io::{prelude::*, BufReader},
    {
    io::{prelude::*, BufReader},
    net::{{TcpListener, TcpStream},
};

use actix_web::dev::Response, TcpStream}
};

// func request handler
// @ Function handle_connection: takes a request and returns a response

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

// func initalize server function
// @ Function initalize_server: takes a port number and returns a server


// func route request
//@  define route for GET request
//@  define route for POST request

// func main function
// @ call initalize_server function
// @ listen for incoming connections

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
}