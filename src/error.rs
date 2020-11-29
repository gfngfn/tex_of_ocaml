
use crate::parser;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(parser::Error)
}

pub fn of_io_error(e: std::io::Error) -> Error {
    Error::IoError(e)
}

pub fn of_parse_error(e: parser::Error) -> Error {
    Error::ParseError(e)
}
