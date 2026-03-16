use status_code::StatusCode;
use std::convert::TryFrom;
use std::net::TcpListener;

use crate::http::{parse_error, request, response, status_code, Request, Response};
use std::io::{Read, Write};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_reques(&mut self, e: &parse_error) -> Response {
        println!("There is error in parsing request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        let listner = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on address: {}", self.addr);
        loop {
            match listner.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(bytes_read) => {
                            println!(
                                "Received a request: {}",
                                String::from_utf8_lossy(&buf[..bytes_read])
                            );
                            let response = match Request::try_from(&buf[..bytes_read]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_reques(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        }
                        Err(e) => println!("Error in reading buffer {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection! {}", e),
            };
        }
    }
}
