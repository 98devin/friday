
mod parsing;
mod ir;

pub use parsing::parser;
pub use parsing::ast;

use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let parse_sequence = parser::SequenceParser::new();

    println!("tgif");

    for arg in args.iter() {
        println!("--- {} ---", arg);
        let file_text = match fs::read_to_string(arg) {
            Ok(file_text) => file_text,
            Err(e) => {
                println!("file error: {}", e);
                continue;
            }
        };

        let decls = match parse_sequence.parse(&file_text) {
            Ok(decls) => decls,
            Err(e) => {
                println!("parse error: {}", e);
                continue;
            }
        };

        for decl in decls.iter() {
            println!("{:?}", decl);
        }
    }
}
