//modules and the structs inside them themselves have different visibility
use super::method::Method;   
use std::convert::TryFrom; 
use std::error::Error;
use std::fmt::{Formatter,Display, Result as FmtResult};

pub struct Request {
    path: String,
    query_string:Option <String>,
    //super keyword tells module to go up a directory to parent and see other children that match declaration
    //method: super::method::Method,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self,String> {

    }
}

//Try to convert a byte slice into our request struct
impl TryFrom<&[u8]> for Request {
    type Error = String;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        ///Shuts compiler up but if this function is called Rust program will panic and exit.
        let string= String::from("asd");
        // string.encrypt();
        // buffer.encrypt();
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        let message = match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        };
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//Error trait implementation from std
impl Error for ParseError {}

//Traits are interfaces that have to be implemented.
trait Encrypt {
    fn Encrypt(&self) -> Self;
}

//Implementation for Encrypt trait
impl Encrypt for String {
    fn Encrypt(&self) -> Self {
        unimplemented!();
    }
}