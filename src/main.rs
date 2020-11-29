/* use std::path::PathBuf; */
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
    input: Option<String>,

    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let opts = Opts::parse();
/*
    let matches =
        App::new("tex_of_ocaml")
            .version("0.1.0")
            .arg(
                Arg::with_name("input")
                    .index(1)
                    .required(false)
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .takes_value(true)
                    .required(false)
            )
            .get_matches();
*/
    let input = opts.input.unwrap_or("(no input)".to_string());
    let output = opts.output.unwrap_or("(no output)".to_string());
    println!("Hello, world! (input: {}, output: {})", input, output);
}
