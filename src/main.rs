use clap::Parser;

use core::{generators::code_gen::{CodeGen, Langs}, parser};
use std::{fs, io::Write, path::Path};

mod core;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// .iris file to use for code generation
    src: String,

    /// Directory to use for code generation output.
    #[arg(short, long, default_value_t = String::from("."))]
    out: String,

    /// Language to use for code generation. Supported: rust.
    #[arg(short, long, default_value_t = String::from("rust"))]
    lang: String,
}

fn main() {
    let args = Args::parse();

    let src = fs::read_to_string(args.src).unwrap();
    let mut parser = parser::Parser::new(src);
    
    parser.generate_ast();

    parser.print();

    let lang = Langs::from_string(&args.lang);
    let mut out = fs::File::create(
        Path::new(args.out.as_str()).join(format!("iris.{}", lang.ext()))
    ).unwrap();

    out.write_all(match lang {
        Langs::RUST(l) => l.gen_code(parser.ast.packages.first().unwrap()),
        Langs::PYTHON(_) => todo!("Python code generation is not yet supported."),
        Langs::CPP(_) => todo!("C++ code generation is not yet supported."),
        Langs::C(_) => todo!("C code generation is not yet supported.")
    }.as_bytes()).unwrap();
}