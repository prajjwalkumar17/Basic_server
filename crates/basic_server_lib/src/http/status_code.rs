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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_values() {
        assert_eq!(StatusCode::Ok as u16, 200);
        assert_eq!(StatusCode::BadRequest as u16, 400);
        assert_eq!(StatusCode::NotFound as u16, 404);
    }

    #[test]
    fn test_status_code_reason_phrase_ok() {
        assert_eq!(StatusCode::Ok.reason_phrase(), "OK");
    }

    #[test]
    fn test_status_code_reason_phrase_bad_request() {
        assert_eq!(StatusCode::BadRequest.reason_phrase(), "Bad Request");
    }

    #[test]
    fn test_status_code_reason_phrase_not_found() {
        assert_eq!(StatusCode::NotFound.reason_phrase(), "Not Found");
    }

    #[test]
    fn test_status_code_into_u16() {
        let code: u16 = StatusCode::Ok.into();
        assert_eq!(code, 200);

        let code: u16 = StatusCode::BadRequest.into();
        assert_eq!(code, 400);

        let code: u16 = StatusCode::NotFound.into();
        assert_eq!(code, 404);
    }

    #[test]
    fn test_status_code_display() {
        assert_eq!(format!("{}", StatusCode::Ok), "200");
        assert_eq!(format!("{}", StatusCode::BadRequest), "400");
        assert_eq!(format!("{}", StatusCode::NotFound), "404");
    }

    #[test]
    fn test_status_code_equality() {
        assert_eq!(StatusCode::Ok, StatusCode::Ok);
        assert_ne!(StatusCode::Ok, StatusCode::NotFound);
    }

    #[test]
    fn test_status_code_copy() {
        let code1 = StatusCode::Ok;
        let code2 = code1;
        assert_eq!(code1, code2);
    }

    #[test]
    fn test_status_code_clone() {
        let code1 = StatusCode::Ok;
        let code2 = code1.clone();
        assert_eq!(code1, code2);
    }
}
