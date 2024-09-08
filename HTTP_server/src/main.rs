//TODO: add actix_web library
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
// func request handler
// @ Function handles_get_request: takes a request and returns a response
// @ Function handles_post_request: takes a request and returns a response
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
}
