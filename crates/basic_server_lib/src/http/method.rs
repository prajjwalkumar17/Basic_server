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

    #[test]
    fn test_method_from_str_get() {
        assert_eq!("GET".parse::<Method>(), Ok(Method::GET));
    }

    #[test]
    fn test_method_from_str_delete() {
        assert_eq!("DELETE".parse::<Method>(), Ok(Method::DELETE));
    }

    #[test]
    fn test_method_from_str_post() {
        assert_eq!("POST".parse::<Method>(), Ok(Method::POST));
    }

    #[test]
    fn test_method_from_str_put() {
        assert_eq!("PUT".parse::<Method>(), Ok(Method::PUT));
    }

    #[test]
    fn test_method_from_str_head() {
        assert_eq!("HEAD".parse::<Method>(), Ok(Method::HEAD));
    }

    #[test]
    fn test_method_from_str_connect() {
        assert_eq!("CONNECT".parse::<Method>(), Ok(Method::CONNECT));
    }

    #[test]
    fn test_method_from_str_options() {
        assert_eq!("OPTIONS".parse::<Method>(), Ok(Method::OPTIONS));
    }

    #[test]
    fn test_method_from_str_trace() {
        assert_eq!("TRACE".parse::<Method>(), Ok(Method::TRACE));
    }

    #[test]
    fn test_method_from_str_patch() {
        assert_eq!("PATCH".parse::<Method>(), Ok(Method::PATCH));
    }

    #[test]
    fn test_method_from_str_invalid() {
        assert!("INVALID".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_lowercase_invalid() {
        assert!("get".parse::<Method>().is_err());
    }

    // Noob test cases - basic edge cases

    #[test]
    fn test_method_from_str_empty() {
        assert!("".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_whitespace() {
        assert!(" ".parse::<Method>().is_err());
        assert!("  ".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_mixed_case() {
        assert!("Get".parse::<Method>().is_err());
        assert!("PoSt".parse::<Method>().is_err());
        assert!("GeT".parse::<Method>().is_err());
        assert!("delete".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_numeric() {
        assert!("123".parse::<Method>().is_err());
        assert!("GET1".parse::<Method>().is_err());
        assert!("1GET".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_special_chars() {
        assert!("GET!".parse::<Method>().is_err());
        assert!("@GET".parse::<Method>().is_err());
        assert!("GET-POST".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_with_leading_trailing_space() {
        assert!(" GET".parse::<Method>().is_err());
        assert!("GET ".parse::<Method>().is_err());
        assert!(" GET ".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_long_string() {
        let long_string = "A".repeat(1000);
        assert!(long_string.parse::<Method>().is_err());
    }

    #[test]
    fn test_method_from_str_unicode() {
        assert!("GÉT".parse::<Method>().is_err());
        assert!("Gët".parse::<Method>().is_err());
    }

    #[test]
    fn test_method_display_get() {
        assert_eq!(format!("{}", Method::GET), "GET");
    }

    #[test]
    fn test_method_display_delete() {
        assert_eq!(format!("{}", Method::DELETE), "DELETE");
    }

    #[test]
    fn test_method_display_post() {
        assert_eq!(format!("{}", Method::POST), "POST");
    }

    #[test]
    fn test_method_display_put() {
        assert_eq!(format!("{}", Method::PUT), "PUT");
    }

    #[test]
    fn test_method_display_head() {
        assert_eq!(format!("{}", Method::HEAD), "HEAD");
    }

    #[test]
    fn test_method_display_connect() {
        assert_eq!(format!("{}", Method::CONNECT), "CONNECT");
    }

    #[test]
    fn test_method_display_options() {
        assert_eq!(format!("{}", Method::OPTIONS), "OPTIONS");
    }

    #[test]
    fn test_method_display_trace() {
        assert_eq!(format!("{}", Method::TRACE), "TRACE");
    }

    #[test]
    fn test_method_display_patch() {
        assert_eq!(format!("{}", Method::PATCH), "PATCH");
    }

    #[test]
    fn test_method_equality() {
        assert_eq!(Method::GET, Method::GET);
        assert_ne!(Method::GET, Method::POST);
    }

    #[test]
    fn test_method_error_display() {
        let err = MethodError;
        assert_eq!(format!("{}", err), "Invalid HTTP method");
    }
}
