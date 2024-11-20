use clap::Parser;

use core::{generators::code_gen::{CodeGen, Langs}, parser, kronos_code_gen};
use std::{collections::HashMap, fs, io::Write, path::Path};

mod core;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// .iris file to use for code generation
    src: String,

    /// Directory to use for code generation output.
    #[arg(short, long, default_value_t = String::from("."))]
    out: String,

    /// Language to use for code generation. Supported: rust, c++, python.
    #[arg(short, long, default_value_t = String::from("rust"))]
    lang: String,

    /// If set, a kronos-code-gen.json file will be created in the same directory as the .iris file
    /// You can ignore this flag, it only exists because is needed by kronos: https://github.com/PoliBa-Space-Program/kronos
    #[arg(short, long)]
    kronos_code_gen: bool
}

fn main() {
    let args = Args::parse();

    let src = fs::read_to_string(args.src).unwrap();
    let mut parser = parser::Parser::new(src);
    parser.generate_ast();

    let lang = Langs::from_string(&args.lang);
    let mut out = fs::File::create(
        Path::new(&args.out).join(format!("iris.{}", lang.ext()))
    ).unwrap();

    out.write_all(match lang {
        Langs::RUST(l) => l.gen_code(&parser.ast.package),
        Langs::PYTHON(l) => l.gen_code(&parser.ast.package),
        Langs::CPP(l) => l.gen_code(&parser.ast.package)
    }.as_bytes()).unwrap();

    if args.kronos_code_gen {
        let mut s = kronos_code_gen::Schema {
            version: String::new(),
            package: String::new(),
            data: HashMap::new(),
            rust_default: HashMap::new()
        };
        s.create_schema(args.out, &parser.ast);
    }
}