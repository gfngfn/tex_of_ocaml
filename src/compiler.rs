use im::hashmap::HashMap;

use crate::syntax::{Expr, Ident, Instruction};

type Level = i32;

type LevelMap = HashMap<Ident, Level>;

#[derive(Debug)]
pub enum Error {
    UnboundVariable(Ident),
    BugOfUnknownArity(i32),
}

pub trait Compilation {
    type Target;
    fn compile(self, lev: Level, levmap: &LevelMap) -> Result<Self::Target, Error>;
}

impl Compilation for Expr {
    type Target = Vec<Instruction>;

    fn compile(self, lev: Level, levmap: &LevelMap) -> Result<Self::Target, Error> {
        match self {
            Expr::Var(x) => match levmap.get(&x) {
                Some(lev_x) => Ok(vec![Instruction::Access(lev - lev_x)]),
                None => Err(Error::UnboundVariable(x)),
            },
            Expr::Lambda(x, e0) => {
                let mut instrs0 = e0.compile(lev + 1, &levmap.update(x, lev + 1))?;
                instrs0.push(Instruction::Return);
                Ok(vec![Instruction::Closure(instrs0)])
            }
            Expr::Apply(e1, e2) => {
                let mut instrs1 = e1.compile(lev, levmap)?;
                let mut instrs2 = e2.compile(lev, levmap)?;
                instrs1.append(&mut instrs2);
                instrs1.push(Instruction::Apply);
                Ok(instrs1)
            }
            Expr::If(e0, e1, e2) => {
                let mut instrs0 = e0.compile(lev, levmap)?;
                let instrs1 = e1.compile(lev, levmap)?;
                let instrs2 = e2.compile(lev, levmap)?;
                instrs0.push(Instruction::If(instrs1, instrs2));
                Ok(instrs0)
            }
            Expr::Const(c) => Ok(vec![Instruction::Const(c)]),
            Expr::Primitive(prim) => match prim.arity() {
                1 => {
                    let sub = vec![
                        Instruction::Access(0),
                        Instruction::Primitive(prim),
                        Instruction::Return,
                    ];
                    Ok(vec![Instruction::Closure(sub)])
                }
                2 => {
                    let app = vec![
                        Instruction::Access(1),
                        Instruction::Access(0),
                        Instruction::Primitive(prim),
                        Instruction::Return,
                    ];
                    let inner = vec![Instruction::Closure(app), Instruction::Return];
                    let outer = vec![Instruction::Closure(inner)];
                    Ok(outer)
                }
                arity => Err(Error::BugOfUnknownArity(arity)),
            },
        }
    }
}

pub fn compile(e: Expr) -> Result<Vec<Instruction>, Error> {
    e.compile(0, &LevelMap::new())
}
