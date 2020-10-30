



fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    server.run();
}


struct Server {
    addr: String,
}

impl Server {
    //Self and server are the same 
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }


    fn run(&self) {
        println!("Listening on {}",self.addr);
    }

}


/* Example request 
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/