//! HTTP request methods

use std::fmt::{Display, Formatter, Result as FmtResult};
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

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Method::GET => write!(f, "GET"),
            Method::DELETE => write!(f, "DELETE"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::HEAD => write!(f, "HEAD"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
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
