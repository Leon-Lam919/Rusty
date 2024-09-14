use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};


// func request handler
// @ Function handle_connection: checking for request and sending response

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = if request_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, len, content);
    stream.write_all(response.as_bytes()).unwrap();
}


// func route request
//@  define route for GET request
//@  define route for POST request

// func main function
// @ call initalize_server function
// @ listen for incoming connections

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}