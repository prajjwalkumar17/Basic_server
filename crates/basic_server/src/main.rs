//! Basic HTTP Server Binary
//!
//! A simple HTTP server that serves static files from a public directory.

use std::env;
use std::fs;

use tracing::{info, warn};
use tracing_subscriber::fmt::time::LocalTime;

use basic_server_lib::{Handler, Method, Request, Response, Server, StatusCode};

/// Website handler that serves static files
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    /// Create a new website handler
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    /// Read a file from the public directory
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(&path) {
            Ok(canonical_path) => {
                if canonical_path.starts_with(&self.public_path) {
                    fs::read_to_string(canonical_path).ok()
                } else {
                    warn!(
                        requested_path = %file_path,
                        resolved_path = ?canonical_path,
                        "Directory traversal attempt blocked"
                    );
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>Hello there!</h1>".to_string()),
                ),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

fn main() {
    // Initialize tracing subscriber with local time formatting
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Default to the public directory at the workspace root
    let default_path = format!(
        "{}/../../public",
        env!("CARGO_MANIFEST_DIR")
    );
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    info!(
        public_path = %public_path,
        bind_address = "127.0.0.1:8080",
        "Starting HTTP server"
    );
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
