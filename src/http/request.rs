//modules and the structs inside them themselves have different visibility
use super::method::Method;   
use std::convert::TryFrom; 
use std::error::Error;
use std::str;
use std::fmt::{Formatter,Display, Result as FmtResult,Debug};
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string:Option <String>,
    //super keyword tells module to go up a directory to parent and see other children that match declaration
    //method: super::method::Method,
    method: Method,
}

//Try to convert a byte slice into our request struct
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        //Shuts compiler up but if this function is called Rust program will panic and exit.
        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }
        
        let request: &str = str::from_utf8(buffer)?;
        // string.encrypt();
        // buffer.encrypt();
        unimplemented!();
    }
}

//Read the incoming request until / then reread frm that point onwards
//If the entire request is send return none, done through Option wrapper.
fn brkdwn_request(request: &str) -> Option<(&str,&str)>{
    for (i,c) in request.chars().enumerate() {
        if c == ' ' {
            //case to skip spaces, normally index+1 in Rust is a bad idea but here we know character is one byte long
            //since it is not cyrillic or an emoji
            return Some((&request[..i],&request[i+1..]));
        }
    }
    //Else return None
    None
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
        return message;
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
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