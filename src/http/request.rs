use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    //GET /user?id=10 HTTP/1.1\r\n eg string to parse

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!() // Supress compiler error
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}
