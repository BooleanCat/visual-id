use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Overflow,
    Parse,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::Parse
    }
}
