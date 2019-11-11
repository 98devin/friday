
use crate::ir::{ModuleTable, ModuleRef, NameTable};

#[derive(Debug, Clone)]
pub struct Context {
    pub global_modl: ModuleRef,
    pub modules: ModuleTable,
    pub idents: NameTable,
}

impl Context {
    pub fn new() -> Context {
        let idents  = NameTable::new();
        let mut modules = ModuleTable::new();

        let (global_modl, _) = modules.new_module("<global>".to_owned());

        Context { global_modl, modules, idents }
    }
}