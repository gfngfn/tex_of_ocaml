use clap::{App, Arg};

fn main() {
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
    let input = matches.value_of("input").unwrap_or("(no input)");
    let output = matches.value_of("output").unwrap_or("(no output)");
    println!("Hello, world! (input: {}, output: {})", input, output);
}
