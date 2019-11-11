
use crate::ir::{ModuleTable, NameTable};

#[derive(Debug, Clone)]
pub struct Context {
    pub modules: ModuleTable,
    pub idents: NameTable,
}

impl Context {
    pub fn new() -> Context {
        Context {
            modules: ModuleTable::new(),
            idents: NameTable::new(),
        }
    }
}