#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: &str) -> Self {
        Ident(s.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub enum Const {
    Int(i32),
    String(String),
    Bool(bool),
}

#[derive(PartialEq, Debug)]
pub enum Primitive {
    Add,
    Sub,
    Mult,
    Append,
    Arabic,
    IsZero,
}

impl Primitive {
    pub fn arity(&self) -> i32 {
        match self {
            Primitive::Add => 2,
            Primitive::Sub => 2,
            Primitive::Mult => 2,
            Primitive::Append => 2,
            Primitive::Arabic => 1,
            Primitive::IsZero => 1,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Ident, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Const(Const),
    Primitive(Primitive),
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Access(i32),
    Closure(Vec<Instruction>),
    Return,
    Apply,
    If(Vec<Instruction>, Vec<Instruction>),
    Const(Const),
    Primitive(Primitive),
}
