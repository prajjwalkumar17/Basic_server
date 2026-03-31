//! HTTP response handling

use std::collections::HashMap;
use std::io::{Result as IOResult, Write};

use super::StatusCode;

/// HTTP header separator
const HEADER_SEPARATOR: &str = ": ";

/// HTTP line ending
const CRLF: &str = "\r\n";

/// HTTP Response
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl Response {
    /// Create a new response with the given status code and optional body
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { 
            status_code, 
            body,
            headers: HashMap::new(),
        }
    }

    /// Get the status code of this response
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Add a header to the response
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Send the response over the provided stream
    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        
        // Build headers string
        let headers_str: String = self.headers
            .iter()
            .map(|(k, v)| format!("{}{HEADER_SEPARATOR}{}{CRLF}", k, v))
            .collect();
        
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            headers_str,
            body.len(),
            body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_new_without_body() {
        let response = Response::new(StatusCode::Ok, None);
        assert_eq!(response.status_code(), StatusCode::Ok);
    }

    #[test]
    fn test_response_new_with_body() {
        let body = Some("Hello, World!".to_string());
        let response = Response::new(StatusCode::Ok, body);
        assert_eq!(response.status_code(), StatusCode::Ok);
    }

    #[test]
    fn test_response_status_code_bad_request() {
        let response = Response::new(StatusCode::BadRequest, None);
        assert_eq!(response.status_code(), StatusCode::BadRequest);
    }

    #[test]
    fn test_response_status_code_not_found() {
        let response = Response::new(StatusCode::NotFound, None);
        assert_eq!(response.status_code(), StatusCode::NotFound);
    }

    #[test]
    fn test_response_with_header() {
        let response = Response::new(StatusCode::Ok, None)
            .with_header("Content-Type", "application/json");
        assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_response_with_multiple_headers() {
        let response = Response::new(StatusCode::Ok, None)
            .with_header("Content-Type", "application/json")
            .with_header("Cache-Control", "no-cache");
        assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(response.headers.get("Cache-Control"), Some(&"no-cache".to_string()));
    }

    #[test]
    fn test_response_send_ok_no_body() {
        let mut buffer = Vec::new();
        let response = Response::new(StatusCode::Ok, None);
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(output.contains("Content-Length: 0"));
        assert!(output.ends_with("\r\n\r\n"));
    }

    #[test]
    fn test_response_send_ok_with_body() {
        let mut buffer = Vec::new();
        let body = Some("Hello".to_string());
        let response = Response::new(StatusCode::Ok, body);
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(output.contains("Content-Length: 5"));
        assert!(output.ends_with("\r\n\r\nHello"));
    }

    #[test]
    fn test_response_send_bad_request() {
        let mut buffer = Vec::new();
        let response = Response::new(StatusCode::BadRequest, None);
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.starts_with("HTTP/1.1 400 Bad Request\r\n"));
    }

    #[test]
    fn test_response_send_not_found() {
        let mut buffer = Vec::new();
        let response = Response::new(StatusCode::NotFound, None);
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn test_response_send_with_headers() {
        let mut buffer = Vec::new();
        let response = Response::new(StatusCode::Ok, None)
            .with_header("X-Custom-Header", "test-value");
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("X-Custom-Header: test-value\r\n"));
    }

    #[test]
    fn test_response_send_json_content_type() {
        let mut buffer = Vec::new();
        let body = Some(r#"{"status":"ok"}"#.to_string());
        let response = Response::new(StatusCode::Ok, body)
            .with_header("Content-Type", "application/json");
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Content-Type: application/json\r\n"));
        assert!(output.contains(r#"{"status":"ok"}"#));
    }

    #[test]
    fn test_response_empty_body_length() {
        let mut buffer = Vec::new();
        let response = Response::new(StatusCode::Ok, Some("".to_string()));
        response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Content-Length: 0"));
    }
}
