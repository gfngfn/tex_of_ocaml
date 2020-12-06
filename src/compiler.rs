use im::hashmap::HashMap;

use crate::list::{append, cons, List};
use crate::syntax::{Expr, Ident, Instruction};

type Level = i32;

type LevelMap = HashMap<Ident, Level>;

#[derive(Debug)]
pub enum Error {
    UnboundVariable(Ident),
}

pub fn compile(e: Expr) -> Result<List<Instruction>, Error> {
    iter(0, &LevelMap::new(), e)
}

fn iter(lev: Level, levmap: &LevelMap, e: Expr) -> Result<List<Instruction>, Error> {
    match e {
        Expr::Var(x) => levmap
            .get(&x)
            .map(|xlev| List::singleton(Instruction::Access(lev - xlev)))
            .ok_or_else(|| Error::UnboundVariable(x)),

        Expr::Lambda(x, e0) => {
            let instrs0 = iter(lev + 1, &levmap.update(x, lev + 1), *e0)?;
            Ok(List::singleton(Instruction::Closure(Box::new(append(
                instrs0,
                List::singleton(Instruction::Return),
            )))))
        }

        Expr::Apply(e1, e2) => {
            let instrs1 = iter(lev, levmap, *e1)?;
            let instrs2 = iter(lev, levmap, *e2)?;
            Ok(append(
                instrs1,
                append(instrs2, List::singleton(Instruction::Apply)),
            ))
        }

        Expr::Const(c) => Ok(List::singleton(Instruction::Const(c))),

        Expr::Primitive(prim) => {
            /* Currently handle arity-2 primitives only.*/
            let app: List<Instruction> = cons(
                Instruction::Access(1),
                cons(
                    Instruction::Access(0),
                    List::singleton(Instruction::Primitive(prim)),
                ),
            );
            let inner =
                Instruction::Closure(Box::new(append(app, List::singleton(Instruction::Return))));
            let outer =
                Instruction::Closure(Box::new(cons(inner, List::singleton(Instruction::Return))));
            Ok(List::singleton(outer))
        }
    }
}
