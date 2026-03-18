//! HTTP request methods

use std::str::FromStr;

/// HTTP request methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodError),
        }
    }
}

/// Error type for invalid HTTP methods
pub struct MethodError;

impl std::fmt::Display for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid HTTP method")
    }
}

impl std::fmt::Debug for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid HTTP method")
    }
}

impl std::error::Error for MethodError {}
