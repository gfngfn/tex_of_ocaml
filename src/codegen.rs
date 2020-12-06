use crate::list::List;
use crate::syntax::{Const, Instruction};

const PREFIX: &str = "\\secd@";
const DOCUMENT_CLASS: &str = "jsarticle";
const VM_PACKAGE_NAME: &str = "secd";
const RESULT_NAME: &str = "result";

pub fn output(instrs: List<Instruction>) -> String {
    let code: String = iter(instrs);
    format!("\\documentclass{{{}}}\\usepackage{{{}}}\\makeatletter\\edef\\{}{{{}EvalAndShow{{{}}}}}\\makeatother\\begin{{document}}\\{}\\end{{document}}",
        DOCUMENT_CLASS, VM_PACKAGE_NAME, RESULT_NAME,
        PREFIX, code, RESULT_NAME
    )
}

fn iter(instrs: List<Instruction>) -> String {
    instrs.foldl(accumulate, "".to_string())
}

fn accumulate(acc: String, instr: Instruction) -> String {
    let s: String = match instr {
        Instruction::Access(i) => {
            let stars = (0..i).map(|_| "*").collect::<String>();
            format!("{}ACCESS{{{}}}", PREFIX, stars)
        }
        Instruction::Closure(instrs_sub) => {
            let s_sub = iter(*instrs_sub);
            format!("{}CLOSURE{{{}}}", PREFIX, s_sub)
        }
        Instruction::Return => format!("{}RETURN{{}}", PREFIX),
        Instruction::Apply => format!("{}APPLY{{}}", PREFIX),
        Instruction::If(instrs1, instrs2) => {
            let s1 = iter(*instrs1);
            let s2 = iter(*instrs2);
            format!("{}IF{{{{{}}}{{{}}}}}", PREFIX, s1, s2)
        }
        Instruction::Const(c) => match c {
            Const::Int(n) => {
                let os = (0..n).map(|_| "o").collect::<String>();
                format!("{}CONST{{{}INT{{{}}}{}ENDVAL}}", PREFIX, PREFIX, os, PREFIX)
            }
            Const::String(s) => format!(
                "{}CONST{{{}STRING{{{}}}{}ENDVAL}}",
                PREFIX, PREFIX, s, PREFIX
            ),
            Const::Bool(b) => {
                let tag = if b { "T" } else { "F" };
                format!(
                    "{}CONST{{{}BOOL{{{}}}{}ENDVAL}}",
                    PREFIX, PREFIX, tag, PREFIX
                )
            }
        },
        Instruction::Primitive(prim) => {
            let cmd = prim.command();
            format!("{}PRIM{{{}{}}}", PREFIX, PREFIX, cmd)
        }
    };
    acc + &s
}
