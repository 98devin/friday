extern crate bumpalo;
extern crate derive_more;
extern crate lalrpop_util;

mod ast;
mod ctx;
mod error;
mod id;
mod ir;
mod refs;
mod storage;

pub use ast::parser;

use bumpalo::Bump;

use ctx::*;
use ir::*;
use refs::*;
use storage::*;

fn main() {
    println!("tgif");

    let arena = Bump::new();
    let ctx = Context::new(&arena);

    for arg in std::env::args().skip(1) {
        println!("--- {} ---", arg);
        let modl_ref = match ctx.process_file(&arg) {
            Ok(modl_ref) => modl_ref,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let ir = ctx.ir.borrow();
        let modl = ir.modl.get(modl_ref).expect("no ir for module");
        println!("{}: {:?}", &modl.name, modl);
    }

    println!("--- all modules: ---");
    for (_modl_ref, modl) in &ctx.ir.borrow().modl {
        println!("{}: {:?}", &modl.name, modl);
    }
}

pub fn verify_file_path(path: &std::path::Path) -> error::Result<String> {
    let file_ext = path.extension().and_then(std::ffi::OsStr::to_str);
    let file_stem = path.file_stem().and_then(std::ffi::OsStr::to_str);

    if file_ext != Some("fri") {
        let path_str = path.to_str().unwrap();
        return Err(error::FridayError::InvalidFilename(path_str.to_owned()))?;
    }

    Ok(file_stem.unwrap().to_owned())
}

impl ctx::Context<'_> {
    pub fn process_file(&self, file_name: &str) -> error::Result<ModlRef> {
        use crate::ast::OwnedToken;
        use crate::parser::SequenceParser;
        use std::fs;
        use std::path::Path;

        let parse_sequence = SequenceParser::new();
        let file_path = Path::new(file_name);

        let modl_name = verify_file_path(file_path)?;

        let file_text = fs::read_to_string(file_name)?;

        let decls = match parse_sequence.parse(self.arena, &file_text) {
            Ok(decls) => decls,
            Err(e) => Err(e.map_token(OwnedToken::from))?,
        };

        let mut refs = self.refs.borrow_mut();
        let mut ast = self.ast.borrow_mut();
        let mut ir = self.ir.borrow_mut();

        let modl_ref = refs.modl.make_ref();

        let new_modl = {
            let decls = self.arena.alloc_slice_copy(&decls);
            ast.modl.set(modl_ref, ast::Modl::ModExp(decls));
            ir.modl.set(modl_ref, ir::Modl::new(modl_name))
        };

        for decl in &decls {
            match decl {
                ast::Decl::Def(sig, _) => {
                    let mut ir_sig = Vec::new();
                    for sign in *sig {
                        ir_sig.push(sign.from_ast(self).forget());
                    }
                    let decl_ref = refs.decl.make_ref();
                    ast.decl.set(decl_ref, *decl);
                    new_modl.symbols.new_decl(decl_ref, ir_sig);
                    new_modl.decls.push(decl_ref);
                }
                ast::Decl::Con(sig) => {
                    let mut ir_sig = Vec::new();
                    for sign in *sig {
                        ir_sig.push(sign.from_ast(self).forget());
                    }
                    let cons_ref = refs.cons.make_ref();
                    ast.cons.set(cons_ref, *decl);
                    new_modl.symbols.new_cons(cons_ref, ir_sig);
                    new_modl.cons.push(cons_ref);
                }
                other => println!("deferring: {}", other),
            }
        }

        Ok(modl_ref)
    }
}
