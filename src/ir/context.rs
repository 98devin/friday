use crate::ir::{Module, ModuleData, ModuleRef, NameTable};

#[derive(Debug, Clone)]
pub struct Context {
    pub global_modl: ModuleRef,
    pub modules: ModuleData,
    pub idents: NameTable,
}

#[derive(Debug, Copy, Clone)]
pub struct WithContext<'ctx, T> {
    pub ctx: &'ctx Context,
    pub val: T,
}

impl<'ctx, T> std::ops::Deref for WithContext<'ctx, T> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl Context {
    pub fn new() -> Context {
        let idents = NameTable::new();
        let mut modules = ModuleData::new();
        let global_modl = modules.make_ref();

        Context {
            global_modl,
            modules,
            idents,
        }
    }

    pub fn modules(&self) -> &ModuleData {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut ModuleData {
        &mut self.modules
    }

    pub fn idents(&self) -> &NameTable {
        &self.idents
    }

    pub fn idents_mut(&mut self) -> &mut NameTable {
        &mut self.idents
    }

    pub fn global_modl(&self) -> ModuleRef {
        self.global_modl
    }

    pub fn wrap<T>(&self, t: T) -> WithContext<'_, T> {
        WithContext { ctx: self, val: t }
    }
}
