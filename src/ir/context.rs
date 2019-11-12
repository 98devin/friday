use crate::ir::{Ident, Module, ModuleRef, ModuleTable, NameTable, Storage, StorageMut};

#[derive(Debug, Clone)]
pub struct Context {
    global_mod: ModuleRef,
    modules: ModuleTable,
    idents: NameTable,
}

impl Context {
    pub fn new() -> Context {
        let idents = NameTable::new();
        let mut modules = ModuleTable::new();

        let (global_mod, _) = modules.new_module("<global>".to_owned());

        Context {
            global_mod,
            modules,
            idents,
        }
    }

    pub fn modules(&self) -> &ModuleTable {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut ModuleTable {
        &mut self.modules
    }

    pub fn global_mod_ref(&self) -> ModuleRef {
        self.global_mod
    }
}

impl Storage<Module> for Context {
    type Ref = ModuleRef;
    fn get(&self, mod_ref: ModuleRef) -> &Module {
        self.modules.get(mod_ref)
    }
}

impl StorageMut<Module> for Context {
    type RefMut = ModuleRef;
    fn get_mut(&mut self, mod_ref: ModuleRef) -> &mut Module {
        self.modules.get_mut(mod_ref)
    }
}

impl Storage<String> for Context {
    type Ref = Ident;
    fn get(&self, id: Ident) -> &String {
        self.idents.get(id)
    }
}
