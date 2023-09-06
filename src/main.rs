fn main() {
    let r_server = rustHttpServer::RustHttpServer::new(String::from("127.0.0.1:1995"));
    r_server.run();
}

mod rustHttpServer {
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
}


mod httpModule  {
    mod reqestModule {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::methodModule::Method
        }
    }
    
    mod methodModule {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
        }
    }
}
