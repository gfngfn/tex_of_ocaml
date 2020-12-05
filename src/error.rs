use crate::parser;

#[derive(Debug)]
pub enum Error<'a> {
    IoError(std::io::Error),
    ParseError(parser::Error<'a>),
}

impl<'a> Error<'a> {
    pub fn of_io_error(e: std::io::Error) -> Self {
        Error::IoError(e)
    }

    pub fn of_parse_error(e: parser::Error<'a>) -> Self {
        Error::ParseError(e)
    }
}
