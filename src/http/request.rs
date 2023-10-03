//modules and the structs inside them themselves have different visibility
use super::method::Method;   
pub struct Request {
    path: String,
    query_string:Option <String>,
    //super keyword tells module to go up a directory to parent and see other children that match declaration
    //method: super::method::Method,
     method: Method,
}