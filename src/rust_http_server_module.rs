use std::net::TcpListener;


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

        let listener = TcpListener::bind(&self.address);
    }
}