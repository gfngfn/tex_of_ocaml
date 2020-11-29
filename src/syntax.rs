
#[derive(Debug)]
pub struct Ident(String);

pub fn ident_of_string(s: String) -> Ident {
    Ident(s)
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Ident, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
}
