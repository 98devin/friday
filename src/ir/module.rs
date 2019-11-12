use std::collections::HashMap;

use crate::ir::{Expr, Ident, Storage, StorageMut, SymbolTable};

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    scope: Vec<ModuleRef>,
    body: Vec<Expr>,
    child: HashMap<Ident, ModuleRef>,
    symbols: SymbolTable,
}

impl Module {
    fn new(name: String) -> Module {
        Module {
            name,
            scope: Vec::new(),
            body: Vec::new(),
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

impl std::ops::Deref for Module {
    type Target = SymbolTable;
    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}

impl std::ops::DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.symbols
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
            modules: Vec::new(),
        }
    }

    pub fn new_module(&mut self, name: String) -> (ModuleRef, &mut Module) {
        let modl_ref = ModuleRef(self.modules.len());
        self.modules.push(Module::new(name));
        (modl_ref, self.modules.last_mut().unwrap())
    }

    pub fn new_module_with_parent(
        &mut self,
        name: String,
        parent: ModuleRef,
    ) -> (ModuleRef, &mut Module) {
        let (modl_ref, modl) = self.new_module(name);
        modl.add_to_scope(parent);
        (modl_ref, modl)
    }
}

impl Storage<Module> for ModuleTable {
    type Ref = ModuleRef;
    fn get(&self, mod_ref: ModuleRef) -> &Module {
        &self.modules[mod_ref.0]
    }
}

impl StorageMut<Module> for ModuleTable {
    type RefMut = ModuleRef;
    fn get_mut(&mut self, mod_ref: ModuleRef) -> &mut Module {
        &mut self.modules[mod_ref.0]
    }
}

impl IntoIterator for ModuleTable {
    type Item = Module;
    type IntoIter = std::vec::IntoIter<Module>;
    fn into_iter(self) -> Self::IntoIter {
        self.modules.into_iter()
    }
}

impl<'a> IntoIterator for &'a ModuleTable {
    type Item = &'a Module;
    type IntoIter = std::slice::Iter<'a, Module>;
    fn into_iter(self) -> Self::IntoIter {
        self.modules.iter()
    }
}

impl<'a> IntoIterator for &'a mut ModuleTable {
    type Item = &'a mut Module;
    type IntoIter = std::slice::IterMut<'a, Module>;
    fn into_iter(self) -> Self::IntoIter {
        self.modules.iter_mut()
    }
}
