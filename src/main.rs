#![allow(dead_code)]

use rust_http_server_module::RustHttpServer;
use std::env;

mod rust_http_server_module;
mod http;
mod website_handler;


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path); 
    let r_server = RustHttpServer::new(String::from("127.0.0.1:1995"));
    r_server.run(website_handler::WebsiteHandler::new(public_path));
}
