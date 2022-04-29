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
    Bool(bool),
}

#[derive(Debug)]
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

    pub fn command(&self) -> &str {
        match self {
            Primitive::Add => "PrimitiveIntAdd",
            Primitive::Sub => "PrimitiveIntSub",
            Primitive::Mult => "PrimitiveIntMult",
            Primitive::Append => "PrimitiveStringAppend",
            Primitive::Arabic => "PrimitiveArabic",
            Primitive::IsZero => "PrimitiveIntIsZero",
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Lambda(Ident, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Const(Const),
    Primitive(Primitive),
}

#[derive(Debug)]
pub enum Instruction {
    Access(i32),
    Closure(Vec<Instruction>),
    Return,
    Apply,
    If(Vec<Instruction>, Vec<Instruction>),
    Const(Const),
    Primitive(Primitive),
}
