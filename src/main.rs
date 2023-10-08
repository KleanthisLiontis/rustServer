use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {

   
    //Strings are UTF8 encoded might take more than one byte unlike ASCII.
    //let server_ip = String::from("127.0.0.1:8080");
    //So this here means take everything after 10nth byte not bit. 
    //et server_port_slice = &server_ip[10..14];
    //immutable
    // let string_borrow: &str = &server_ip;
    // let string_literal = "123";
    // dbg!(&server_ip);
    // dbg!(server_port_slice);
    
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}