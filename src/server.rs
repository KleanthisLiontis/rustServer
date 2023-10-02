//Server struct and implementation everything members and methods private by default

pub struct Server {
    address: String,
    }

impl Server {
    pub fn new(address: String) -> Server{//can also be Self as this returns an object of type Server
        Server { 
            address 
        }
    }

    //We can make it &mut self or &self so we can change ownership
    pub fn run (self) {
         println!("Server running and listening on {}.", self.address);
    }
}