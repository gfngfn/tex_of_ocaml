use clap::Clap;
use std::fs;

mod compiler;
mod error;
mod list;
mod parser;
mod syntax;

use error::Error;

#[derive(Clap, Debug)]
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
    display(&opts);
    run(opts);
}

fn display(opts: &Opts) {
    let input: &String = &opts.input;
    let output: Option<&String> = opts.output.as_ref();
    println!("Hello, world! (input: {:?}, output: {:?})", input, output);
}

fn run(opts: Opts) {
    let input_path: &String = &opts.input;
    match fs::read_to_string(input_path).map_err(Error::of_io_error) {
        Err(err) => println!("Error: {:?}", err),

        Ok(s) => {
            let input = &s;
            match parser::parse(input).map_err(Error::of_parse_error) {
                Ok(e) => {
                    println!("Expression: {:?}", e);
                    match compiler::compile(e) {
                        Ok(instrs) => {
                            println!("Instruction: {:?}", instrs);
                        }
                        Err(err) => println!("Error: {:?}", err),
                    }
                }
                Err(err) => println!("Error: {:?}", err),
            }
        }
    }
}
