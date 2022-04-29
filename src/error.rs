use crate::compiler;
use crate::parser;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseError(parser::Error),
    CompilationError(compiler::Error),
}

impl Error {
    pub fn of_io_error(e: std::io::Error) -> Self {
        Error::IoError(e)
    }

    pub fn of_parse_error(e: parser::Error) -> Self {
        Error::ParseError(e)
    }

    pub fn of_compilation_error(e: compiler::Error) -> Self {
        Error::CompilationError(e)
    }
}
