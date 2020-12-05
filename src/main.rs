use clap::Clap;
use std::fs;

mod error;
mod parser;
mod syntax;

use error::Error;
use syntax::Expr;

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
    let res = run(opts);
    if let Err(e) = res {
        println!("Error: {:?}", e);
    }
}

fn display(opts: &Opts) {
    let input: &String = &opts.input;
    let output: Option<&String> = opts.output.as_ref();
    println!("Hello, world! (input: {:?}, output: {:?})", input, output);
}

fn run(opts: Opts) -> Result<(), Error<'static>> {
    let input_path: &String = &opts.input;
    let s: String = fs::read_to_string(input_path).map_err(Error::of_io_error)?;
    let e: Expr = parser::parse(&s).map_err(Error::of_parse_error)?;
    println!("Content: {:?}", e);
    Ok(())
}
