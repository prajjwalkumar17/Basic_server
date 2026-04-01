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

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock handler for testing
    struct MockHandler;

    impl Handler for MockHandler {
        fn handle_request(&mut self, _request: &Request) -> Response {
            Response::new(StatusCode::Ok, Some("Mock response".to_string()))
        }
    }

    #[test]
    fn test_handle_bad_request_returns_400() {
        let mut handler = MockHandler;
        let response = handler.handle_bad_request(&ParseError::InvalidRequest);
        assert_eq!(response.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn test_handle_bad_request_invalid_encoding() {
        let mut handler = MockHandler;
        let response = handler.handle_bad_request(&ParseError::InvalidEncoding);
        assert_eq!(response.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn test_handle_bad_request_invalid_protocol() {
        let mut handler = MockHandler;
        let response = handler.handle_bad_request(&ParseError::InvalidProtocol);
        assert_eq!(response.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn test_handle_bad_request_invalid_method() {
        let mut handler = MockHandler;
        let response = handler.handle_bad_request(&ParseError::InvalidMethod);
        assert_eq!(response.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn test_handle_bad_request_no_body() {
        let mut handler = MockHandler;
        let response = handler.handle_bad_request(&ParseError::InvalidRequest);
        // Verify response has no body by checking the Content-Length in output
        let mut buffer = Vec::new();
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Content-Length: 0"));
    }

    #[test]
    fn test_handler_trait_object() {
        let mut handler: Box<dyn Handler> = Box::new(MockHandler);
        let raw_request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        let response = handler.handle_request(&request);
        assert_eq!(response.status_code(), StatusCode::Ok);
    }

    #[test]
    fn test_mock_handler_returns_ok() {
        let mut handler = MockHandler;
        let raw_request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        let response = handler.handle_request(&request);
        assert_eq!(response.status_code(), StatusCode::Ok);
    }
}
