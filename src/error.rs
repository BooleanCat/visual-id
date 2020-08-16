use std::num::ParseIntError;
use png::EncodingError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Overflow,
    Parse,
    Encode,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::Parse
    }
}

impl From<EncodingError> for Error {
    fn from(e: EncodingError) -> Self {
        println!("{:?}", e);
        Error::Encode
    }
}