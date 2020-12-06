use crate::list::List;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Ident(String);

impl Ident {
    pub fn of_string(s: String) -> Self {
        Ident(s)
    }
}

#[derive(Debug)]
pub enum Const {
    Int(i32),
    String(String),
}

#[derive(Debug)]
pub enum Primitive {
    Add,
    Sub,
    Mult,
    Append,
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Ident, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Const(Const),
    Primitive(Primitive),
}

#[derive(Debug)]
pub enum Instruction {
    Access(i32),
    Closure(Box<List<Instruction>>),
    Return,
    Apply,
    Const(Const),
    Primitive(Primitive),
}
