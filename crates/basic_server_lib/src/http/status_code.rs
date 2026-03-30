//! HTTP status codes

use std::fmt::{Display, Formatter, Result as FmtResult};

/// HTTP status codes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    /// Get the reason phrase for this status code
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(status: StatusCode) -> u16 {
        status as u16
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
