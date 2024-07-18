use clap::Parser;

use core::{parser, token_types::TokenTypes, tokenizer::{Token, Tokenizer}};
use std::fs;

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


    //parse(&args.src, &args.out, &args.lang);
    /*let src = fs::read_to_string(args.src).unwrap();

    let mut tokenizer = Tokenizer::new(src);
    tokenizer.tokenize();

    for t in tokenizer.tokens {
        println!("{:?} {:?}", t.t, t.value);
    }*/

    let src = fs::read_to_string(args.src).unwrap();
    let mut parser = parser::Parser::new(src);
    parser.generate_ast();

    for p in parser.ast.packages {
        println!("{:?} {:?}", p.name, p.version);
        
        for s in p.structs.values() {
            println!("{:?}:", s.name);
            for f in s.fields.values() {
                println!("{:?} {:?} [{:?}]", f.name, f.t, f.array);
            }
        }

        for e in p.enums.values() {
            println!("{:?}:", e.name);
            for v in &e.variants_order {
                println!("{:?} {:?}", v.name, v.value)
            }
        }
    }
}