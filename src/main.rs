#![allow(dead_code)]

use rust_http_server_module::RustHttpServer;
use std::env;

mod rust_http_server_module;
mod http;
mod website_handler;


fn main() {
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let r_server = RustHttpServer::new(String::from("127.0.0.1:1995"));
    r_server.run(website_handler::WebsiteHandler::new(public_path));
}
