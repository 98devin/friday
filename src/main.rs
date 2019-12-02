extern crate bumpalo;
extern crate derive_more;
extern crate lalrpop_util;

mod ast;
mod ctx;
mod error;
mod id;
mod ir;
mod phases;
mod refs;
mod storage;

pub use ast::parser;

use bumpalo::Bump;

use ctx::*;
use ir::*;

fn main() {
    match _main() {
        Ok(()) => (),
        Err(err) => eprintln!("{}", err),
    }
}

fn _main() -> error::Result<()> {
    println!("tgif");

    let arena = Bump::new();
    let ctx = Context::new(&arena);

    for arg in std::env::args().skip(1) {
        println!("--- {} ---", arg);
        phases::process_file(&ctx, &arg)?;
    }

    println!("--- resolving aliases ---");
    phases::process_aliases(&ctx)?;

    println!("--- all modules: ---");
    let ir = ctx.ir.borrow();
    for (modl_ref, modl) in &ir.modl {
        println!("{:?} = {:?}", modl_ref, modl);
    }

    Ok(())
}
