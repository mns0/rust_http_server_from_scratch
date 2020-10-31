use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    //Self and server are the same
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        let _listener = TcpListener::bind(&self.addr).unwrap();
        //infinite loop! :) same as while true

        loop {
            match _listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            };
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
