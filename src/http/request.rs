use crate::http::request;
use super::method::{MethodError,Method};   
use super::{QueryString,QueryStringValue};
use std::convert::TryFrom; 
use std::error::Error;
use std::str;
use std::fmt::{Formatter,Display, Result as FmtResult,Debug};
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'life_buf> {
    path: &'life_buf str,
    query_string:Option <QueryString<'life_buf>>,
    //super keyword tells module to go up a directory to parent and see other children that match declaration
    //method: super::method::Method,
    method: Method,
}

impl<'life_buf> Request<'life_buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

//Try to convert a byte slice into our request struct, had to explicity state lifetimes
impl<'life_buf> TryFrom<&'life_buf [u8]> for Request<'life_buf> {
    type Error = ParseError;

    fn try_from(buffer: &'life_buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }
        
        /*Check if received request matched expectations
        First check string matches format we want:
        GET /search?name=abc&sort=1 HTTP/1.1 */
        let request = str::from_utf8(buffer)?;
        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        } 
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
        // string.encrypt();
        // buffer.encrypt();
    }
}

//Read the incoming request until / then reread frm that point onwards
//If the entire request is send return none, done through Option wrapper.
fn get_next_word(request: &str) -> Option<(&str,&str)>{
    //go char by char and index them
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            //case to skip spaces and r, normally index+1 in Rust is a bad idea but here we know character is one byte long
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

//Error trait implementation from std
impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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