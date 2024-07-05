use std::env;


use core::parser::parse;

mod core;


fn main() {
    let args: Vec<String> = env::args().collect();    
    
    if args.len() != 2 {
        panic!("Excpected file path.");
    }
    
    parse(&args[1]);
}