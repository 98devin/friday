
pub mod module;
pub mod definition;
pub mod identifier;
pub mod symbol;
pub mod context;

pub use module::{Module, ModuleTable, ModuleRef};
pub use definition::{Decl, Data, Expr, Literal, Patn};
pub use identifier::{Ident, NameTable};
pub use symbol::{SymbolTable, DataRef, DeclRef};
pub use context::{Context};

use crate::error::FridayError::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub fn initial_context() -> Context {
    Context::new()
}

pub fn verify_file_path(path: &std::path::Path) -> Result<String> {

    let file_ext  = path.extension().and_then(std::ffi::OsStr::to_str);
    let file_stem = path.file_stem().and_then(std::ffi::OsStr::to_str);

    if file_ext != Some("fri") {
        let path_str = path.to_str().unwrap();
        return Err(InvalidFilename(path_str.to_owned()))?
    }
    
    Ok(file_stem.unwrap().to_owned())
}

pub fn process_file(ctx: &mut Context, file_name: &str) -> Result<ModuleRef> {
    use std::path::Path;
    use std::fs;

    use crate::parser::SequenceParser;
    
    let parse_sequence = SequenceParser::new();
    
    let file_path = Path::new(file_name);

    let modl_name = verify_file_path(file_path)?;

    let file_text = fs::read_to_string(file_name)?;

    use crate::parsing::OwnedToken;
    let _decls = match parse_sequence.parse(&file_text) {
        Ok(decls) => decls,
        Err(e) => Err(e.map_token(OwnedToken::from))?,
    };

    let (modl_ref, modl) = ctx.modules.new_module(modl_name);

    modl.add_to_scope(ctx.global_modl);

    Ok(modl_ref)

}
