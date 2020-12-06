use crate::list::List;
use crate::syntax::{Const, Instruction};

pub fn output(instrs: List<Instruction>) -> String {
    iter(instrs)
}

fn iter(instrs: List<Instruction>) -> String {
    instrs.foldl(accumulate, "".to_string())
}

fn accumulate(acc: String, instr: Instruction) -> String {
    let prefix = "\\secd@";
    let s: String = match instr {
        Instruction::Access(i) => {
            let stars = (0..i).map(|_| "*").collect::<String>();
            format!("{}ACCESS{{{}}}", prefix, stars)
        }
        Instruction::Closure(instrs_sub) => {
            let s_sub = iter(*instrs_sub);
            format!("{}CLOSURE{{{}}}", prefix, s_sub)
        }
        Instruction::Return => format!("{}RETURN{{}}", prefix),
        Instruction::Apply => format!("{}APPLY{{}}", prefix),
        Instruction::Const(c) => match c {
            Const::Int(n) => {
                let os = (0..n).map(|_| "o").collect::<String>();
                format!("{}CONST{{{}INT{{{}}}{}ENDVAL}}", prefix, prefix, os, prefix)
            }
            Const::String(s) => format!(
                "{}CONST{{{}STRING{{{}}}{}ENDVAL}}",
                prefix, prefix, s, prefix
            ),
        },
    };
    acc + &s
}
