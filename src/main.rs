mod error;
mod ir;
mod parsing;

pub use parsing::ast;
pub use parsing::parser;

use ir::{Module, ModuleRef, Storage, StorageMut};

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

        let modl: &Module = ctx.get(modl_ref);
        println!("{}: {:?}", &modl.name(), modl);
    }
}
