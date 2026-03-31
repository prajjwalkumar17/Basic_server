//! HTTP request parsing

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

use super::method::{Method, MethodError};
use super::QueryString;

/// HTTP Request
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    /// Get the request path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the request method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get the query string
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

/// Parse errors for HTTP requests
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::query_string::Value;

    #[test]
    fn test_request_valid_get() {
        let raw_request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/");
        assert_eq!(request.method(), &Method::GET);
    }

    #[test]
    fn test_request_valid_post() {
        let raw_request = b"POST /submit HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/submit");
        assert_eq!(request.method(), &Method::POST);
    }

    #[test]
    fn test_request_valid_delete() {
        let raw_request = b"DELETE /resource HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/resource");
        assert_eq!(request.method(), &Method::DELETE);
    }

    #[test]
    fn test_request_valid_put() {
        let raw_request = b"PUT /update HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/update");
        assert_eq!(request.method(), &Method::PUT);
    }

    #[test]
    fn test_request_valid_head() {
        let raw_request = b"HEAD /page HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/page");
        assert_eq!(request.method(), &Method::HEAD);
    }

    #[test]
    fn test_request_valid_connect() {
        let raw_request = b"CONNECT server:443 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "server:443");
        assert_eq!(request.method(), &Method::CONNECT);
    }

    #[test]
    fn test_request_valid_options() {
        let raw_request = b"OPTIONS * HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "*");
        assert_eq!(request.method(), &Method::OPTIONS);
    }

    #[test]
    fn test_request_valid_trace() {
        let raw_request = b"TRACE / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/");
        assert_eq!(request.method(), &Method::TRACE);
    }

    #[test]
    fn test_request_valid_patch() {
        let raw_request = b"PATCH /partial HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/partial");
        assert_eq!(request.method(), &Method::PATCH);
    }

    #[test]
    fn test_request_with_query_string() {
        let raw_request = b"GET /search?q=rust HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/search");
        assert!(request.query_string().is_some());
        let qs = request.query_string().unwrap();
        match qs.get("q") {
            Some(Value::Single(v)) => assert_eq!(*v, "rust"),
            _ => panic!("Expected query string value"),
        }
    }

    #[test]
    fn test_request_without_query_string() {
        let raw_request = b"GET /page HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/page");
        assert!(request.query_string().is_none());
    }

    #[test]
    fn test_request_query_string_multiple_params() {
        let raw_request = b"GET /api?name=john&age=30 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = Request::try_from(raw_request.as_slice()).unwrap();
        assert_eq!(request.path(), "/api");
        let qs = request.query_string().unwrap();
        match qs.get("name") {
            Some(Value::Single(v)) => assert_eq!(*v, "john"),
            _ => panic!("Expected name value"),
        }
        match qs.get("age") {
            Some(Value::Single(v)) => assert_eq!(*v, "30"),
            _ => panic!("Expected age value"),
        }
    }

    #[test]
    fn test_request_invalid_encoding() {
        let raw_request: &[u8] = &[0xFF, 0xFE, 0x00];
        let result = Request::try_from(raw_request);
        assert!(matches!(result, Err(ParseError::InvalidEncoding)));
    }

    #[test]
    fn test_request_invalid_protocol() {
        let raw_request = b"GET / HTTP/1.0\r\nHost: localhost\r\n\r\n";
        let result = Request::try_from(raw_request.as_slice());
        assert!(matches!(result, Err(ParseError::InvalidProtocol)));
    }

    #[test]
    fn test_request_invalid_method() {
        let raw_request = b"INVALID / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let result = Request::try_from(raw_request.as_slice());
        assert!(matches!(result, Err(ParseError::InvalidMethod)));
    }

    #[test]
    fn test_request_malformed_missing_path() {
        let raw_request = b"GET\r\nHost: localhost\r\n\r\n";
        let result = Request::try_from(raw_request.as_slice());
        // The parser finds "GET" then splits on \r, so it returns InvalidProtocol
        // because the protocol is parsed as "localhost" (from "\nHost:")
        assert!(result.is_err());
    }

    #[test]
    fn test_request_malformed_missing_protocol() {
        let raw_request = b"GET /\r\nHost: localhost\r\n\r\n";
        let result = Request::try_from(raw_request.as_slice());
        // The parser finds "GET", then "/", then "\nHost:" as protocol
        // which is not "HTTP/1.1", so it returns InvalidProtocol
        assert!(matches!(result, Err(ParseError::InvalidProtocol)));
    }

    #[test]
    fn test_request_malformed_empty() {
        let raw_request = b"";
        let result = Request::try_from(raw_request.as_slice());
        assert!(matches!(result, Err(ParseError::InvalidRequest)));
    }

    #[test]
    fn test_parse_error_display() {
        assert_eq!(format!("{}", ParseError::InvalidRequest), "Invalid Request");
        assert_eq!(format!("{}", ParseError::InvalidEncoding), "Invalid Encoding");
        assert_eq!(format!("{}", ParseError::InvalidProtocol), "Invalid Protocol");
        assert_eq!(format!("{}", ParseError::InvalidMethod), "Invalid Method");
    }

    #[test]
    fn test_parse_error_from_method_error() {
        let err: ParseError = MethodError.into();
        assert!(matches!(err, ParseError::InvalidMethod));
    }

    #[test]
    fn test_parse_error_from_utf8_error() {
        let utf8_err = std::str::from_utf8(&[0xFF, 0xFE]).unwrap_err();
        let err: ParseError = utf8_err.into();
        assert!(matches!(err, ParseError::InvalidEncoding));
    }

    #[test]
    fn test_get_next_word() {
        assert_eq!(get_next_word("hello world"), Some(("hello", "world")));
        assert_eq!(get_next_word("hello\rworld"), Some(("hello", "world")));
        assert_eq!(get_next_word("hello"), None);
    }
}
