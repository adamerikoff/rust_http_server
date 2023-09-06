use std::{fmt::Display, net::TcpStream};
use std::io::{Write, Result as IoResult};
use super::status_code_module;

#[derive(Debug)]

pub struct Response {
    status_code: status_code_module::StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: status_code_module::StatusCode, body: Option<String>) -> Self {
        Response { 
            status_code: status_code, 
            body: body 
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}