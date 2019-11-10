mod parsing;
use parsing::parser;

mod ir;

use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let parse_sequence = parser::SequenceParser::new();

    println!("tgif");

    for arg in args.iter() {
        println!("--- {} ---", arg);
        let file_text = match fs::read_to_string(arg) {
            Err(e) => {
                println!("{}", e);
                continue;
            }
            Ok(file_text) => file_text,
        };

        let decls = match parse_sequence.parse(&file_text) {
            Err(e) => {
                println!("{}", e);
                continue;
            }
            Ok(decls) => decls,
        };

        for decl in decls.iter() {
            println!("{:?}", decl);
        }
    }
}
