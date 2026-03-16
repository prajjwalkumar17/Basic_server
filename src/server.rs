use std::convert::TryFrom;
use std::net::TcpListener;

use crate::http::{parse_error, Request, Response, StatusCode};
use std::io::{Read, Write};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &parse_error) -> Response {
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
                            let request = Request::try_from(&buf[..bytes_read]);

                            let response = match request {
                                Ok(request) => {
                                    let response = handler.handle_request(&request);
                                    println!(
                                        "[router] {} {} -> {} {}",
                                        request.method(),
                                        request.path(),
                                        response.status_code(),
                                        response.status_code().reason_phrase(),
                                    );
                                    response
                                }
                                Err(e) => handler.handle_bad_request(&e),
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
