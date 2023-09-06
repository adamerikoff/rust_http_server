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
        println!("Listening on {}", self.address)
    }
}