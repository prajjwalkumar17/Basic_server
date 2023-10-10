#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use server::Server;
// use http::Request;
use website_handler::WebsiteHandler;
use http::Method;
use std::env;
mod website_handler;
mod http;
mod server;
fn main() {
    let default_path=format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let public_path=env::var("PUBLIC_PATH").unwrap_or(default_path);
    let get=Method::GET;
    println!("Starting server");
    let server=Server::new("127.0.01:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}   
