use super::constants::{METHOD_CONNECT, METHOD_DELETE, METHOD_GET, METHOD_HEAD, METHOD_OPTIONS, METHOD_PATCH, METHOD_POST, METHOD_PUT, METHOD_TRACE};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug)]
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
    type Err = method_error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            METHOD_GET => Ok(Method::GET),
            METHOD_DELETE => Ok(Method::DELETE),
            METHOD_POST => Ok(Method::POST),
            METHOD_PUT => Ok(Method::PUT),
            METHOD_HEAD => Ok(Method::HEAD),
            METHOD_CONNECT => Ok(Method::CONNECT),
            METHOD_OPTIONS => Ok(Method::OPTIONS),
            METHOD_TRACE => Ok(Method::TRACE),
            METHOD_PATCH => Ok(Method::PATCH),
            _ => Err(method_error),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::GET => write!(f, "{}", METHOD_GET),
            Self::DELETE => write!(f, "{}", METHOD_DELETE),
            Self::POST => write!(f, "{}", METHOD_POST),
            Self::PUT => write!(f, "{}", METHOD_PUT),
            Self::HEAD => write!(f, "{}", METHOD_HEAD),
            Self::CONNECT => write!(f, "{}", METHOD_CONNECT),
            Self::OPTIONS => write!(f, "{}", METHOD_OPTIONS),
            Self::TRACE => write!(f, "{}", METHOD_TRACE),
            Self::PATCH => write!(f, "{}", METHOD_PATCH),
        }
    }
}

pub struct method_error;
