//Server struct and implementation everything members and methods private by default
use std::net::TcpListener;

pub struct Server {
    address: String,
    }

impl Server {
    pub fn new(address: String) -> Self{//can also be Self as this returns an object of type Server
        Self { 
            address 
        }
    }

    //We can make it &mut self or &self so we can change ownership
    pub fn run (self) {
         println!("Server running and listening on {}.", self.address);

         let tcp_listener = TcpListener::bind(&self.address).unwrap();
    }
}