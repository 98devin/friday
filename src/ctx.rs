use crate::ast::AstStorage;
use crate::id::NameTable;
use crate::ir::IrStorage;
use crate::refs::*;

use bumpalo::Bump;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Context<'ctx> {
    pub arena: &'ctx Bump,
    pub refs: RefCell<IdCounter>,
    global_modl: ModlRef,
    pub names: RefCell<NameTable<'ctx>>,
    pub ast: RefCell<AstStorage<'ctx>>,
    pub ir: RefCell<IrStorage>,
}

#[derive(Debug, Copy, Clone)]
pub struct WithContext<'ctx, T> {
    pub ctx: &'ctx Context<'ctx>,
    pub val: T,
}

impl<'ctx, T> WithContext<'ctx, T> {
    pub fn wrap<U>(&self, u: U) -> WithContext<'ctx, U> {
        WithContext {
            ctx: self.ctx,
            val: u,
        }
    }
}

impl<'ctx, T> std::ops::Deref for WithContext<'ctx, T> {
    type Target = Context<'ctx>;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'ctx> Context<'ctx> {
    pub fn new(arena: &'ctx Bump) -> Self {
        let mut refs = IdCounter::new();
        let global_modl = refs.modl.make_ref();

        Context {
            arena,
            global_modl,
            refs: RefCell::new(refs),
            names: RefCell::new(NameTable::new()),
            ast: RefCell::new(AstStorage::new()),
            ir: RefCell::new(IrStorage::new()),
        }
    }

    pub fn wrap<T>(&'ctx self, t: T) -> WithContext<'ctx, T> {
        WithContext { ctx: self, val: t }
    }

    pub fn global_modl(&self) -> ModlRef {
        self.global_modl
    }
}
