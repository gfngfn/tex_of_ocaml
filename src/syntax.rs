#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn of_string(s: String) -> Self {
        Ident(s)
    }
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Ident, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
}
