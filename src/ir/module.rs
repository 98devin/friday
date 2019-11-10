use std::collections::HashMap;
use std::marker::PhantomData;

use crate::ir;

#[derive(Debug, Clone)]
pub struct Module {
    scope: Vec<ModuleRef>,
    child: HashMap<ir::Ident, ModuleRef>,
}

impl Module {
    fn new() -> Module {
        Module {
            scope: Vec::new(),
            child: HashMap::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModuleRef(usize, *const ModuleManager);

#[derive(Debug, Clone)]
pub struct ModuleManager {
    modules: Vec<Module>,
}

impl ModuleManager {
    pub fn get_module(&self, mod_ref: ModuleRef) -> &Module {
        let self_ptr = self as *const ModuleManager;
        assert_eq!(mod_ref.1, self_ptr);
        &self.modules[mod_ref.0]
    }

    pub fn get_module_mut(&mut self, mod_ref: ModuleRef) -> &mut Module {
        let self_ptr = self as *const ModuleManager;
        assert_eq!(mod_ref.1, self_ptr);
        &mut self.modules[mod_ref.0]
    }

    pub fn new_module(&mut self) -> ModuleRef {
        self.modules.push(Module::new());
        ModuleRef(self.modules.len() - 1, self as *const ModuleManager)
    }
}
