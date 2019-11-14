use derive_more::{From, Into};
use std::collections::HashMap;

use crate::ast;
use crate::ir;
use crate::ir::storage::{RefCounter, RefCounterIter, VecStorage};
use crate::ir::{Expr, Ident, Storage, StorageMut, SymbolTable};

#[derive(Debug, Clone)]
pub struct Module {
    id: ModuleRef,
    name: String,
    scope: Vec<ModuleRef>,
    child: HashMap<Ident, ModuleRef>,
    symbols: SymbolTable,
}

impl Module {
    pub fn new(id: ModuleRef, name: String) -> Module {
        Module {
            id,
            name,
            scope: Vec::new(),
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

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    pub fn symbols_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbols
    }
}

impl std::ops::Deref for Module {
    type Target = SymbolTable;
    fn deref(&self) -> &Self::Target {
        self.symbols()
    }
}

impl std::ops::DerefMut for Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.symbols_mut()
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct ModuleRef(usize);

#[derive(Debug, Clone)]
pub struct ModuleData {
    ref_counter: RefCounter<ModuleRef>,
    pub ast: VecStorage<ast::Modl, ModuleRef>,
    pub ir: VecStorage<ir::Module, ModuleRef>,
}

impl ModuleData {
    pub fn new() -> ModuleData {
        ModuleData {
            ref_counter: RefCounter::new(),
            ast: VecStorage::new(),
            ir: VecStorage::new(),
        }
    }

    pub fn make_ref(&mut self) -> ModuleRef {
        self.ref_counter.make_ref()
    }
}

impl<'a> IntoIterator for &'a ModuleData {
    type Item = ModuleRef;
    type IntoIter = RefCounterIter<ModuleRef>;
    fn into_iter(self) -> Self::IntoIter {
        self.ref_counter.into_iter()
    }
}
