#![allow(dead_code)]

mod rust_http_server_module;
mod http;

use rust_http_server_module::RustHttpServer;

fn main() {
    let r_server = RustHttpServer::new(String::from("127.0.0.1:1995"));
    r_server.run();
}
