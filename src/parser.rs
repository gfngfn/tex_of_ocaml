
use crate::syntax;
use crate::syntax::{Ident, Expr};

#[derive(Debug)]
pub enum Error {
}

pub fn parse(_s: &str) -> Result<Expr, Error> {
    Ok(Expr::Var(Ident::of_string("x".to_string())))
}
