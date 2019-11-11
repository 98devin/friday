
use std::collections::{HashMap};

use crate::ir::{Ident, Expr, SymbolTable};


#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    scope: Vec<ModuleRef>,
    body:  Vec<Expr>,
    child: HashMap<Ident, ModuleRef>,
    symbols: SymbolTable,
}

impl Module {
    fn new(name : String) -> Module {
        Module {
            name,
            scope: Vec::new(),
            body:  Vec::new(),
            child: HashMap::new(),
            symbols: SymbolTable::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn lookup_child(&self, child_id: &Ident) -> Option<ModuleRef> {
        self.child.get(child_id).copied()
    }

    pub fn add_to_scope(&mut self, modl: ModuleRef) {
        self.scope.push(modl);
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModuleRef(usize);

#[derive(Debug, Clone)]
pub struct ModuleTable {
    modules: Vec<Module>,
}

impl ModuleTable {

    pub fn new() -> ModuleTable {
        ModuleTable {
            modules: Vec::new()
        }
    }

    pub fn get_module(&self, mod_ref: ModuleRef) -> &Module {
        &self.modules[mod_ref.0]
    }

    pub fn get_module_mut(&mut self, mod_ref: ModuleRef) -> &mut Module {
        &mut self.modules[mod_ref.0]
    }

    pub fn new_module(&mut self, name: String) -> (ModuleRef, &mut Module) {
        let modl_ref = ModuleRef(self.modules.len());
        self.modules.push(Module::new(name));
        (modl_ref, self.modules.last_mut().unwrap())
    }
}
