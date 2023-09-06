use crate::http::{
    request_module::Request, 
    response_module::Response,
    status_code_module::StatusCode,
    method_module::Method
};

use super::rust_http_server_module::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}


impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("ROOT".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("HELLO WORLD".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),

        }
    }
}