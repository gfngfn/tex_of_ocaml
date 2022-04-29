use crate::syntax::{Const, Instruction};

const PREFIX: &str = "\\secd@";
const DOCUMENT_CLASS: &str = "jsarticle";
const VM_PACKAGE_NAME: &str = "secd";
const RESULT_NAME: &str = "result";

pub trait Serialization {
    fn serialize(self: Self) -> String;
}

impl Serialization for Vec<Instruction> {
    fn serialize(self: Self) -> String {
        let mut code = String::new();
        for instr in self {
            let s = instr.serialize();
            code.push_str(&s);
        }
        code
    }
}

impl Serialization for Instruction {
    fn serialize(self: Self) -> String {
        match self {
            Instruction::Access(i) => {
                let stars = (0..i).map(|_| "*").collect::<String>();
                format!("{}ACCESS{{{}}}", PREFIX, stars)
            }
            Instruction::Closure(instrs) => {
                let sub = instrs.serialize();
                format!("{}CLOSURE{{{}}}", PREFIX, sub)
            }
            Instruction::Return => format!("{}RETURN{{}}", PREFIX),
            Instruction::Apply => format!("{}APPLY{{}}", PREFIX),
            Instruction::If(instrs1, instrs2) => {
                let s1 = instrs1.serialize();
                let s2 = instrs2.serialize();
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
        }
    }
}

pub fn output(instrs: Vec<Instruction>) -> String {
    let code = instrs.serialize();
    format!("\\documentclass{{{}}}\\usepackage{{{}}}\\makeatletter\\edef\\{}{{{}EvalAndShow{{{}}}}}\\makeatother\\begin{{document}}\\{}\\end{{document}}",
        DOCUMENT_CLASS, VM_PACKAGE_NAME, RESULT_NAME,
        PREFIX, code, RESULT_NAME
    )
}
