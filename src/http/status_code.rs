use std::fmt::{Display, Formatter, Result as FmtResult};

//Not gonna implement all the codes here:)))))
#[derive(Copy,Clone,Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

//unsigned u16 due to size used
impl Display for StatusCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", *self as u16)
    }
}
