//! HTTP response handling

use std::io::{Result as IOResult, Write};

use super::StatusCode;

/// HTTP Response
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    /// Create a new response with the given status code and optional body
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    /// Get the status code of this response
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Send the response over the provided stream
    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
