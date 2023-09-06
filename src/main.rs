fn main() {
    let r_server = RustHttpServer::new(String::from("127.0.0.1:1995"));
    r_server.run();
}

struct RustHttpServer {
    address: String,
}

impl RustHttpServer {
    fn new(address: String) -> Self{
        Self {
            address: address
        }
    }

    fn run(self) {
        println!("Listening on {}", self.address)
    }
}

