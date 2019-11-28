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
use refs::ModlRef;
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
        println!("{}: {:?}", modl.name(), modl);
    }

    println!("--- all modules: ---");
    for (modl_ref, modl) in &ctx.ir.borrow().modl {
        println!("{}\t {:?} = {:?}", modl.name(), modl_ref, modl);
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

impl<'ctx> ctx::Context<'ctx> {
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
            Err(e) => Err(e.map_token(OwnedToken::from))?,
            Ok(decls) => decls,
        };

        let decls = self.arena.alloc_slice_copy(&decls);

        let modl_ref = {
            let mut refs = self.refs.borrow_mut();
            let mut ast = self.ast.borrow_mut();
            let mut ir = self.ir.borrow_mut();
            let modl_ref = refs.modl.make_ref();

            ast.modl.set(modl_ref, ast::Modl::ModExp(decls));
            ir.modl.set(modl_ref, ir::Modl::new(modl_name));

            modl_ref
        };

        self.process_modl_init(modl_ref);

        Ok(modl_ref)
    }

    pub fn process_modl_init(&self, modl_ref: ModlRef) {
        /*
            Modules (either in mod or use declarations)
            which need to be processed after this one is.
            Since we borrow the ir, ast, and refs fields,
            we make sure to only recursively process these
            modules once this current call has completed the
            bulk of its work.
        */
        let mut deferred = Vec::new();

        {
            let mut anon_modl_counter = 0;

            let mut ir = self.ir.borrow_mut();
            let mut ast = self.ast.borrow_mut();
            let mut refs = self.refs.borrow_mut();

            let modl_ast = ast.modl.get(modl_ref).expect("Module has no ast!");
            let decls = match modl_ast {
                ast::Modl::ModExp(decls) => decls,
                ast::Modl::Named(_) => return, // TODO: process these later.
            };

            let modl_ir = ir.modl.get_mut(modl_ref).expect("Module has no ir!");
            let record = match modl_ir {
                Modl::Record(record) => record,
                Modl::Alias(_) => return, // TODO: Process these later.
            };

            let modl_name = record.name.clone();

            for decl in decls.iter() {
                match decl {
                    ast::Decl::Def(sig, _) => {
                        let ir_sig: Vec<_> = sig
                            .iter()
                            .map(|sign| sign.from_ast(self).forget())
                            .collect();

                        let decl_ref = refs.decl.make_ref();
                        ast.decl.set(decl_ref, *decl);

                        record.symbols.new_decl(decl_ref, ir_sig);
                        record.decls.push(decl_ref);
                    }
                    ast::Decl::Con(sig) => {
                        let ir_sig: Vec<_> = sig
                            .iter()
                            .map(|sign| sign.from_ast(self).forget())
                            .collect();

                        let cons_ref = refs.cons.make_ref();
                        ast.cons.set(cons_ref, *decl);

                        record.symbols.new_cons(cons_ref, ir_sig);
                        record.cons.push(cons_ref);
                    }
                    ast::Decl::Let(..) => {
                        let let_ref = refs.decl.make_ref();
                        ast.decl.set(let_ref, *decl);
                        record.decls.push(let_ref);
                    }
                    ast::Decl::Mod(id, &ast_modl) => {
                        let name = self.names.borrow_mut().make_ident(id.0);
                        let new_modl = refs.modl.make_ref();
                        record.children.insert(name, new_modl);
                        ast.modl.set(new_modl, ast_modl);

                        deferred.push((
                            format!("{}.{}", modl_name, id.0.to_owned()),
                            new_modl,
                            ast_modl,
                        ));
                    }
                    ast::Decl::Use(&ast_modl) => {
                        let new_modl = refs.modl.make_ref();
                        record.scope.push(new_modl);
                        ast.modl.set(new_modl, ast_modl);
                        deferred.push((
                            format!("{}.<anon{}>", modl_name, anon_modl_counter),
                            new_modl,
                            ast_modl,
                        ));
                        anon_modl_counter += 1;
                    }
                    other => println!("deferring: {}", other),
                }
            }
        }

        /*
            Handle creation of ir and further processing
            for each module ref created as a result of processing
            the outer module.
        */
        for (child_name, child_ref, child_ast) in deferred {
            match child_ast {
                ast::Modl::Named(ast::ModlPath { path, absolute }) => {
                    let mut ir = self.ir.borrow_mut();

                    let parent = if absolute {
                        self.global_modl()
                    } else {
                        modl_ref
                    };

                    let child_modl = ir.modl.set(child_ref, Modl::new_alias(child_name, parent));
                    let alias = child_modl.as_alias();
                    alias.path.extend(
                        path.iter()
                            .map(|id| self.names.borrow_mut().make_ident(id.0)),
                    );
                }
                ast::Modl::ModExp(_) => {
                    self.ir
                        .borrow_mut()
                        .modl
                        .set(child_ref, Modl::new(child_name));
                    self.process_modl_init(child_ref);
                }
            }
        }
    }
}
