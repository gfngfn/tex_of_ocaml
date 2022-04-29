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
    let result = run(opts);
    match result {
        Err(err) => {
            println!("Error: {:?}", err)
        }
        Ok(did_output) => {
            if did_output {
                println!("Output written.")
            } else {
                println!("No output.")
            }
        }
    }
}

fn run(opts: Opts) -> Result<bool, Error> {
    let input_path: &String = &opts.input;
    let input = fs::read_to_string(input_path).map_err(Error::of_io_error)?;
    let e = parser::parse(&input).map_err(Error::of_parse_error)?;
    let instrs = compiler::compile(e).map_err(Error::of_compilation_error)?;
    let code = codegen::output(instrs);
    if let Some(output_path) = opts.output.as_ref() {
        output(output_path, &code)
            .map(|()| true)
            .map_err(Error::of_io_error)
    } else {
        Ok(false)
    }
}

fn output(output_path: &str, code: &str) -> std::io::Result<()> {
    let mut out = fs::File::create(output_path)?;
    out.write_all(code.as_bytes())
}
