use std::net::TcpListener;

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
}