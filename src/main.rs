fn main() {

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let head = Method::HEAD;
    let connect = Method::CONNECT;
    let options = Method::OPTIONS;
    let trace = Method::TRACE;
    let patch = Method::PATCH;

    //Strings are UTF8 encoded might take more than one byte unlike ASCII.
    let server_ip = String::from("127.0.0.1:8080");
    //So this here means take everything after 10nth byte not bit. 
    let server_port_slice = &server_ip[10..14];
    //immutable
    // let string_borrow: &str = &server_ip;
    // let string_literal = "123";

    dbg!(&server_ip);
    dbg!(server_port_slice);
    let server = Server::new(server_ip);
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Server{//can also be Self as this returns an object of type Server
        Server { 
            address 
        }
    }

    //We can make it &mut self or &self so we can change ownership
    fn run (self) {
        println!("Server running and listening on {}.", self.address);
    }
}

struct Request {
    path: String,
    query_string: String,
    method:Method
}

enum Method {
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
/*GET /user?id=10 GTTP/1.1\r\n
HEADERS \r\n
BODY
*/