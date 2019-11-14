pub mod context;
pub mod definition;
pub mod identifier;
pub mod module;
pub mod storage;
pub mod symbol;

pub use context::{Context, WithContext};
pub use definition::{Cons, Decl, Expr, Literal, Patn, Sign};
pub use identifier::{Ident, NameTable};
pub use module::{Module, ModuleData, ModuleRef};
pub use storage::{Storage, StorageMut};
pub use symbol::{ConsRef, DeclRef, SymbolTable};

use crate::error::FridayError::*;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub fn verify_file_path(path: &std::path::Path) -> Result<String> {
    let file_ext = path.extension().and_then(std::ffi::OsStr::to_str);
    let file_stem = path.file_stem().and_then(std::ffi::OsStr::to_str);

    if file_ext != Some("fri") {
        let path_str = path.to_str().unwrap();
        return Err(InvalidFilename(path_str.to_owned()))?;
    }

    Ok(file_stem.unwrap().to_owned())
}

impl Context {
    pub fn process_file(&mut self, file_name: &str) -> Result<ModuleRef> {
        use crate::ast;
        use crate::parser::SequenceParser;
        use crate::parsing::OwnedToken;
        use std::fs;
        use std::path::Path;

        let parse_sequence = SequenceParser::new();
        let file_path = Path::new(file_name);

        let modl_name = verify_file_path(file_path)?;

        let file_text = fs::read_to_string(file_name)?;

        let decls = match parse_sequence.parse(&file_text) {
            Ok(decls) => decls,
            Err(e) => Err(e.map_token(OwnedToken::from))?,
        };

        let modl_ref = self.modules_mut().make_ref();
        let mut new_modl = Module::new(modl_ref, modl_name);
        new_modl.add_to_scope(self.global_modl());

        for decl in &decls {
            match decl {
                ast::Decl::Def(sig, _) => {
                    let ir_sig: Vec<_> =
                        sig.iter().map(|s| self.sign_from_ast(s).forget()).collect();
                    let decl_ref = new_modl.symbols_mut().new_decl(ir_sig);
                    new_modl.symbols_mut().decl.ast.set(decl_ref, decl.clone());
                }
                ast::Decl::Con(sig) => {
                    let ir_sig: Vec<_> =
                        sig.iter().map(|s| self.sign_from_ast(s).forget()).collect();
                    let cons_ref = new_modl.symbols_mut().new_cons(ir_sig);
                    new_modl.symbols_mut().cons.ast.set(cons_ref, decl.clone());
                }
                other => println!("deferring: {}", other),
            }
        }

        let modules = self.modules_mut();
        modules.ast.set(modl_ref, ast::Modl::ModExp(decls));
        modules.ir.set(modl_ref, new_modl);

        Ok(modl_ref)
    }
}
