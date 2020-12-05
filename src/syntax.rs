use crate::list::List;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

#[derive(Debug)]
pub enum Instruction {
    Access(i32),
    Closure(Box<List<Instruction>>),
    Return,
    Apply,
}
