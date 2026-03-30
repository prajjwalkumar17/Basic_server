//! HTTP response handling

use std::collections::HashMap;
use std::io::{Result as IOResult, Write};

use super::StatusCode;

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
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
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
