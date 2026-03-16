use crate::http::Method;

use super::http::{Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!(
                        "Some one tried Directory traversal Attack!!!!!\n The path traversed is {file_path}"
                    );
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        let method = request.method();
        let request_path = request.path();

        match method {
            Method::GET => match request_path {
                "/" => {
                    let body = self.read_file("index.html");
                    let status_code = if body.is_some() {
                        StatusCode::Ok
                    } else {
                        StatusCode::NotFound
                    };
                    Response::new(status_code, body)
                }
                "/hello" => {
                    Response::new(StatusCode::Ok, Some("<h1>hellow there!!!</h1>".to_string()))
                }
                path => {
                    let body = self.read_file(path);
                    let status_code = if body.is_some() {
                        StatusCode::Ok
                    } else {
                        StatusCode::NotFound
                    };
                    Response::new(status_code, body)
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
