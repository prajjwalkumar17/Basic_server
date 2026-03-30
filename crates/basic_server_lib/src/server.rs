//! HTTP Server implementation

use std::io::Read;
use std::net::TcpListener;
use std::time::Instant;

use tracing::{debug, error, info, warn};

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

    /// Generate a simple request ID for tracking
    fn generate_request_id() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        format!("req-{:06}", COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    /// Start the server and handle incoming connections
    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        info!(address = %self.addr, "Server listening");
        
        loop {
            match listener.accept() {
                Ok((mut stream, peer_addr)) => {
                    debug!(peer = ?peer_addr, "New connection accepted");
                    
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(bytes_read) => {
                            let request_id = Self::generate_request_id();
                            let start_time = Instant::now();
                            
                            let request_str = String::from_utf8_lossy(&buf[..bytes_read]);
                            debug!(request_id = %request_id, raw_request = %request_str, "Received raw request");
                            
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    info!(
                                        request_id = %request_id,
                                        method = %request.method(),
                                        path = %request.path(),
                                        "Processing request"
                                    );
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    warn!(request_id = %request_id, error = %e, "Failed to parse request");
                                    handler.handle_bad_request(&e)
                                }
                            };
                            
                            let elapsed = start_time.elapsed();
                            let status = response.status_code();
                            let status_code: u16 = status.into();
                            
                            // Add X-Request-Id header to response
                            let response = response.with_header("X-Request-Id", request_id.clone());
                            
                            // Log at appropriate level based on status code
                            if status_code >= 500 {
                                error!(
                                    request_id = %request_id,
                                    status = status_code,
                                    elapsed_ms = elapsed.as_millis() as u64,
                                    "Request completed with server error"
                                );
                            } else if status_code >= 400 {
                                warn!(
                                    request_id = %request_id,
                                    status = status_code,
                                    elapsed_ms = elapsed.as_millis() as u64,
                                    "Request completed with client error"
                                );
                            } else {
                                info!(
                                    request_id = %request_id,
                                    status = status_code,
                                    elapsed_ms = elapsed.as_millis() as u64,
                                    "Request completed successfully"
                                );
                            }
                            
                            if let Err(e) = response.send(&mut stream) {
                                error!(request_id = %request_id, error = %e, "Failed to send response");
                            }
                        }
                        Err(e) => {
                            error!(error = %e, "Error reading from connection");
                        }
                    };
                }
                Err(e) => {
                    error!(error = %e, "Failed to accept connection");
                }
            };
        }
    }
}
