use std::net::TcpListener;
use std::io::{ Write, Read };
use crate::http::status_code_module;
use crate::http::response_module;
use crate::http::request_module;

pub struct RustHttpServer {
    address: String,
}

impl RustHttpServer {
    pub fn new(address: String) -> Self{
        Self {
            address: address
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener: TcpListener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));                            
                            match request_module::Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = response_module::Response::new(status_code_module::StatusCode::Ok, Some("IT WORKS".to_string()));
                                    response.send(&mut stream);
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error to establish a connection: {}", e);
                }
            }
        }

    }
}