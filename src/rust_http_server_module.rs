use std::net::TcpListener;
use std::io::{ Write, Read };
use crate::http::status_code_module;
use crate::http::response_module;
use crate::http::request_module;

pub trait  Handler {
    fn handle_request(&mut self, request: &request_module::Request) -> response_module::Response;
    
    fn handle_bad_request(&mut self, e: &request_module::ParseError) -> response_module::Response {
        println!("Failed to parse request: {}", e);
        response_module::Response::new(
            status_code_module::StatusCode::BadRequest, 
            None)
    }

}


pub struct RustHttpServer {
    address: String,
}

impl RustHttpServer {
    pub fn new(address: String) -> Self{
        Self {
            address: address
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);

        let listener: TcpListener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));                            
                            let response = match request_module::Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
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