//define public interface for http parent module
pub use method::Method;
pub use request::Request;
pub use request::ParseError;
//pub use response::Response;

pub mod method;
pub mod request;
//pub mod response;