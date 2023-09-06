use crate::http::{request_module::Request, response_module::Response};

use super::rust_http_server_module::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(
            crate::http::status_code_module::StatusCode::Ok,
            Some("TEST".to_string())
        )
    }
}