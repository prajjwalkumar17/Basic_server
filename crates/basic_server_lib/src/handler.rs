//! Request handler trait

use tracing::warn;

use crate::http::{ParseError, Request, Response, StatusCode};

/// Trait for handling HTTP requests
pub trait Handler {
    /// Handle a valid HTTP request
    fn handle_request(&mut self, request: &Request) -> Response;

    /// Handle a malformed request
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        warn!(error = %e, "Handling bad request");
        Response::new(StatusCode::BadRequest, None)
    }
}
