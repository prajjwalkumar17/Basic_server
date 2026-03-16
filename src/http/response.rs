use super::{constants::HTTP_VERSION, StatusCode};
use std::io::{Result as IOResult, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "{} {} {}\r\n\n{}",
            HTTP_VERSION,
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
