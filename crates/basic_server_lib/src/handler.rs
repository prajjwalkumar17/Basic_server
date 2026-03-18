//! Request handler trait

use crate::http::{ParseError, Request, Response, StatusCode};

/// Trait for handling HTTP requests
pub trait Handler {
    /// Handle a valid HTTP request
    fn handle_request(&mut self, request: &Request) -> Response;

    /// Handle a malformed request
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Error parsing request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
