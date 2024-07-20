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

    let src = fs::read_to_string(args.src).unwrap();
    let mut parser = parser::Parser::new(src);
    
    parser.generate_ast();

    parser.print();
}