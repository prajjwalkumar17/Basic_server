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
#[derive(PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test all valid HTTP methods parse correctly
    #[test]
    fn test_method_from_str_valid() {
        let valid_methods = [
            ("GET", Method::GET),
            ("DELETE", Method::DELETE),
            ("POST", Method::POST),
            ("PUT", Method::PUT),
            ("HEAD", Method::HEAD),
            ("CONNECT", Method::CONNECT),
            ("OPTIONS", Method::OPTIONS),
            ("TRACE", Method::TRACE),
            ("PATCH", Method::PATCH),
        ];

        for (input, expected) in valid_methods {
            assert_eq!(input.parse::<Method>(), Ok(expected), "Failed for: {}", input);
        }
    }

    /// Test Display trait for all HTTP methods
    #[test]
    fn test_method_display() {
        let methods = [
            (Method::GET, "GET"),
            (Method::DELETE, "DELETE"),
            (Method::POST, "POST"),
            (Method::PUT, "PUT"),
            (Method::HEAD, "HEAD"),
            (Method::CONNECT, "CONNECT"),
            (Method::OPTIONS, "OPTIONS"),
            (Method::TRACE, "TRACE"),
            (Method::PATCH, "PATCH"),
        ];

        for (method, expected) in methods {
            assert_eq!(format!("{}", method), expected, "Failed for: {:?}", method);
        }
    }

    /// Test invalid method strings are rejected
    #[test]
    fn test_method_from_str_invalid() {
        let invalid_cases = [
            "INVALID",
            "get",           // lowercase
            "",              // empty
            " ",             // whitespace
            "  ",            // multiple whitespace
            "Get",           // mixed case
            "PoSt",          // mixed case
            "GeT",           // mixed case
            "delete",        // lowercase
            "123",           // numeric
            "GET1",          // method with number
            "1GET",          // number prefix
            "GET!",          // special char
            "@GET",          // special char prefix
            "GET-POST",      // combined methods
            " GET",          // leading space
            "GET ",          // trailing space
            " GET ",         // both spaces
            "GÉT",           // unicode
            "Gët",           // unicode
        ];

        for input in invalid_cases {
            assert!(input.parse::<Method>().is_err(), "Should be invalid: {:?}", input);
        }
    }

    /// Test long string is rejected
    #[test]
    fn test_method_from_str_long_string() {
        let long_string = "A".repeat(1000);
        assert!(long_string.parse::<Method>().is_err());
    }

    /// Test method equality
    #[test]
    fn test_method_equality() {
        assert_eq!(Method::GET, Method::GET);
        assert_ne!(Method::GET, Method::POST);
    }

    /// Test MethodError display
    #[test]
    fn test_method_error_display() {
        let err = MethodError;
        assert_eq!(format!("{}", err), "Invalid HTTP method");
    }
}
