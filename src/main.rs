#![allow(dead_code)]
use server::Server;

mod http;
mod server;

fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    server.run();
}

/* Example request
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
