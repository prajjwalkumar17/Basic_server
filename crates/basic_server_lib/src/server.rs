//! HTTP Server implementation

use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;
use crate::handler::Handler;

/// HTTP Server that listens for connections and routes requests to a handler
pub struct Server {
    addr: String,
}

impl Server {
    /// Create a new server bound to the specified address
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    /// Start the server and handle incoming connections
    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on address: {}", self.addr);
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Error reading buffer: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            };
        }
    }
}
