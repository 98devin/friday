
mod error;
mod parsing;
mod ir;

pub use parsing::parser;
pub use parsing::ast;

use std::env;

fn main() {

    println!("tgif");

    let mut ctx = ir::initial_context();

    for arg in env::args().skip(1) {
        println!("--- {} ---", arg);
        
        let modl_ref = match ir::process_file(&mut ctx, &arg) {
            Ok(modl_ref) => modl_ref,
            Err(e) => { eprintln!("{}", e); continue },
        };

        let modl = ctx.modules.get_module(modl_ref);
        println!("{}: {:?}", &modl.name(), modl);

    }
}
