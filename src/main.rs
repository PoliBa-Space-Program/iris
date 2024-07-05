use clap::Parser;

use core::parser::parse;

mod core;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: String,

    #[arg(short, long, default_value_t = String::from("."))]
    out: String,

    #[arg(short, long, default_value_t = String::from("rust"))]
    lang: String,
}

fn main() {
    let args = Args::parse();

    parse(&args.src);
}