use clap::Parser;
use std::fs;
use std::io::prelude::*;

mod codegen;
mod compiler;
mod error;
mod parser;
mod syntax;

use error::Error;

#[derive(Parser, Debug)]
#[clap(
    name = "tex_of_ocaml",
    version = "0.1.0",
    author = "Takashi SUWA",
    about = "A compiler from lambda terms into TeX sources"
)]
struct Opts {
    #[clap(name = "INPUT")]
    input: String,

    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    run(opts);
}

fn run(opts: Opts) {
    let input_path: &String = &opts.input;
    match fs::read_to_string(input_path).map_err(Error::of_io_error) {
        Err(err) => println!("Error: {:?}", err),

        Ok(s) => {
            let input = &s;
            match parser::parse(input).map_err(Error::of_parse_error) {
                Ok(e) => {
                    /* println!("Expression: {:?}", e); */
                    match compiler::compile(e) {
                        Ok(instrs) => {
                            /* println!("Instruction: {:?}", instrs); */
                            let code = codegen::output(instrs);
                            /* println!("Code: {:?}", code); */
                            if let Some(output_path) = opts.output.as_ref() {
                                let res = output(output_path, &code);
                                match res {
                                    Ok(()) => println!("Output written."),
                                    Err(err) => println!("Error: {:?}", err),
                                }
                            } else {
                                println!("No output.")
                            }
                        }
                        Err(err) => println!("Error: {:?}", err),
                    }
                }
                Err(err) => println!("Error: {:?}", err),
            }
        }
    }
}

fn output(output_path: &String, code: &str) -> std::io::Result<()> {
    let mut out = fs::File::create(output_path)?;
    out.write_all(code.as_bytes())
}
