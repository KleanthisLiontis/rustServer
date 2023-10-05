//Rust enums each member can have different type 
use std::str::FromStr;

pub enum Method {
    // GET(String), //= 0
    // DELETE(u64), //=1
    GET,
    DELETE,
    POST, //=2  
    PUT,//=7 rest would increment from here 8-9-10...
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
 }

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
         "GET" => Ok(Self::GET),
         "DELETE" => Ok(Self::DELETE),
         "POST" => Ok(Self::POST),
         "PUT" => Ok(Self::PUT),
         "HEAD" => Ok(Self::HEAD),
         "CONNECT" => Ok(Self::CONNECT),
         "OPTIONS" => Ok(Self::OPTIONS),
         "TRACE" => Ok(Self::TRACE),
         "PATCH" => Ok(Self::PATCH),
         _ => Err(MethodError),
      }
   }
}

pub struct MethodError; 