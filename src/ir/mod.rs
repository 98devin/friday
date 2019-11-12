pub mod context;
pub mod definition;
pub mod identifier;
pub mod module;
pub mod symbol;

pub use context::Context;
pub use definition::{Data, Decl, Expr, Literal, Patn};
pub use identifier::{Ident, NameTable};
pub use module::{Module, ModuleRef, ModuleTable};
pub use symbol::{DataRef, DeclRef, SymbolTable};

use crate::error::FridayError::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub fn verify_file_path(path: &std::path::Path) -> Result<String> {
    let file_ext = path.extension().and_then(std::ffi::OsStr::to_str);
    let file_stem = path.file_stem().and_then(std::ffi::OsStr::to_str);

    if file_ext != Some("fri") {
        let path_str = path.to_str().unwrap();
        return Err(InvalidFilename(path_str.to_owned()))?;
    }
    Ok(file_stem.unwrap().to_owned())
}

pub trait Storage<T> {
    type Ref: Copy + std::hash::Hash;
    fn get(&self, _ref: Self::Ref) -> &T;
}

pub trait StorageMut<T>: Storage<T> {
    type RefMut: Copy + std::hash::Hash;
    fn get_mut(&mut self, _ref: Self::RefMut) -> &mut T;
}

impl Context {
    pub fn process_file(&mut self, file_name: &str) -> Result<ModuleRef> {
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

        let global_mod_ref = self.global_mod_ref();
        let (modl_ref, _modl) = self
            .modules_mut()
            .new_module_with_parent(modl_name, global_mod_ref);

        for decl in &decls {
            match decl {
                other => println!("deferring: {}", other),
            }
        }

        Ok(modl_ref)
    }
}
