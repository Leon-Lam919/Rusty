
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

// func request handler
// @ Function handles_get_request: takes a request and returns a response

// func initalize server function
// @ Function initalize_server: takes a port number and returns a server


// func route request
//@  define route for GET request
//@  define route for POST request

// func main function
// @ call initalize_server function
// @ listen for incoming connections

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request:Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    println!("Request: {http_request:#?}");
}