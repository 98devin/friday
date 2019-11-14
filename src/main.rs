extern crate derive_more;
extern crate lalrpop_util;

mod error;
mod ir;
mod parsing;

pub use parsing::ast;
pub use parsing::parser;

use ir::{Module, Storage};

use std::env;

fn main() {
    println!("tgif");

    let mut ctx = ir::Context::new();

    for arg in env::args().skip(1) {
        println!("--- {} ---", arg);
        let modl_ref = match ctx.process_file(&arg) {
            Ok(modl_ref) => modl_ref,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let modl = ctx.modules().ir.get(modl_ref).expect("no ir for module");
        println!("{}: {:?}", modl.name(), modl);
    }

    println!("--- all modules: ---");
    for (_modl_ref, modl) in &ctx.modules().ir {
        println!("{}: {:?}", modl.name(), modl);
    }
}
