#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;
mod http;
mod server;
mod website_handler;
fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Starting server");
    println!("Starting server");
    let server = Server::new("127.0.01:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
