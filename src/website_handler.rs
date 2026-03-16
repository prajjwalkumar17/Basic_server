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

    fn method_name(method: &Method) -> &'static str {
        match method {
            Method::GET => "GET",
            Method::DELETE => "DELETE",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::HEAD => "HEAD",
            Method::CONNECT => "CONNECT",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH",
        }
    }

    fn respond_with_log(
        &self,
        method: &Method,
        path: &str,
        status_code: StatusCode,
        body: Option<String>,
    ) -> Response {
        println!(
            "[router] {} {} -> {} {}",
            Self::method_name(method),
            path,
            status_code,
            status_code.reason_phrase()
        );
        Response::new(status_code, body)
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
                    self.respond_with_log(method, request_path, status_code, body)
                }
                "/hello" => self.respond_with_log(
                    method,
                    request_path,
                    StatusCode::Ok,
                    Some("<h1>hellow there!!!</h1>".to_string()),
                ),
                path => {
                    let body = self.read_file(path);
                    let status_code = if body.is_some() {
                        StatusCode::Ok
                    } else {
                        StatusCode::NotFound
                    };
                    self.respond_with_log(method, request_path, status_code, body)
                }
            },
            _ => self.respond_with_log(method, request_path, StatusCode::NotFound, None),
        }
    }
}
