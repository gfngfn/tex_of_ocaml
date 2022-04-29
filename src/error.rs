use crate::compiler;
use crate::parser;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parsing(parser::Error),
    Compilation(compiler::Error),
}

impl Error {
    pub fn of_io_error(e: std::io::Error) -> Self {
        Error::Io(e)
    }

    pub fn of_parse_error(e: parser::Error) -> Self {
        Error::Parsing(e)
    }

    pub fn of_compilation_error(e: compiler::Error) -> Self {
        Error::Compilation(e)
    }
}
