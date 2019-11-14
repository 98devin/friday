use derive_more::{From, Into};

use crate::ast;
use crate::ir::storage::{RefCounter, VecStorage};
use crate::ir::{self, Cons, Decl, Sign, Storage, StorageMut};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeclData {
    ref_counter: RefCounter<DeclRef>,
    pub ast: VecStorage<ast::Decl, DeclRef>,
    pub ir: VecStorage<ir::Decl, DeclRef>,
}

impl DeclData {
    pub fn new() -> Self {
        Self {
            ref_counter: RefCounter::new(),
            ast: VecStorage::new(),
            ir: VecStorage::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsData {
    ref_counter: RefCounter<ConsRef>,
    pub ast: VecStorage<ast::Decl, ConsRef>,
    pub ir: VecStorage<ir::Decl, ConsRef>,
}

impl ConsData {
    pub fn new() -> Self {
        Self {
            ref_counter: RefCounter::new(),
            ast: VecStorage::new(),
            ir: VecStorage::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub decl: DeclData,
    pub cons: ConsData,

    decl_signs: HashMap<Vec<Sign>, Vec<DeclRef>>,
    cons_signs: HashMap<Vec<Sign>, Vec<ConsRef>>,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct DeclRef(usize);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct ConsRef(usize);

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            decl: DeclData::new(),
            cons: ConsData::new(),

            decl_signs: HashMap::new(),
            cons_signs: HashMap::new(),
        }
    }

    pub fn iter_decl(&self) -> impl Iterator<Item = DeclRef> {
        self.decl.ref_counter.into_iter()
    }

    pub fn iter_cons(&self) -> impl Iterator<Item = ConsRef> {
        self.cons.ref_counter.into_iter()
    }

    pub fn lookup_decl(&self, sign: &[Sign]) -> &[DeclRef] {
        self.decl_signs.get(sign).map(AsRef::as_ref).unwrap_or(&[])
    }

    pub fn lookup_cons(&self, sign: &[Sign]) -> &[ConsRef] {
        self.cons_signs.get(sign).map(AsRef::as_ref).unwrap_or(&[])
    }

    pub fn new_decl(&mut self, sign: Vec<Sign>) -> DeclRef {
        let decl_ref = self.decl.ref_counter.make_ref();
        use std::collections::hash_map::Entry;
        match self.decl_signs.entry(sign) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(decl_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(decl_ref),
        }

        decl_ref
    }

    pub fn new_cons(&mut self, sign: Vec<Sign>) -> ConsRef {
        let cons_ref = self.cons.ref_counter.make_ref();
        use std::collections::hash_map::Entry;
        match self.cons_signs.entry(sign) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(cons_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(cons_ref),
        }

        cons_ref
    }
}
