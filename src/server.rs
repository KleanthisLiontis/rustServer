//Server struct and implementation everything members and methods private by default
use std::net::TcpListener;
use std::io::Read;

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
        
        loop{
            match tcp_listener.accept() {
                //Can ignore tupple/or each variable returned from ok wrapper by using _ instread of tuple.
                Ok((mut stream,_address)) => {
                    println!("Connections accepted!");
                    let mut buffer = [0; 1024]; //stores 1024bytes for requests
                    match stream.read(& mut buffer){
                        Ok(_) => {
                            //from lossy cannot fail so we dont have to handle errors
                            print!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Error accepting connections! {}", e),
            }

            // let result = tcp_listener.accept();
            // if(result.is_err()) {
            //     continue;
            // }
            // let (stream,address) = result.unwrap();
        };
    }
}