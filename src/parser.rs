
use crate::syntax;
use crate::syntax::{Expr};

#[derive(Debug)]
pub enum Error {
}

pub fn parse(_s: &str) -> Result<Expr, Error> {
    Ok(Expr::Var(syntax::ident_of_string("x".to_string())))
}
