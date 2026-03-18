//! Basic HTTP Server Library
//!
//! A simple HTTP server implementation with request parsing and response handling.

pub mod handler;
pub mod http;
pub mod server;

pub use handler::Handler;
pub use http::{Method, ParseError, QueryString, Request, Response, StatusCode};
pub use server::Server;
