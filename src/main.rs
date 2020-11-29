/* use std::path::PathBuf; */
use std::fs;
/* use std::io::{BufRead, BufReader}; */
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "tex_of_ocaml",
    version = "0.1.0",
    author = "Takashi SUWA",
    about = "A compiler from lambda terms into TeX sources",
)]
struct Opts {
/*    #[clap(name = "INPUT", parse(from_os_str))] */
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
    let input = &opts.input;
    let output = opts.output.as_ref();
    println!("Hello, world! (input: {:?}, output: {:?})", input, output);
}

fn run(opts: Opts) {
    let input_path: &String = &opts.input;
    let res = fs::read_to_string(input_path);
    match res {
        Ok(s)  => println!("Content: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}
