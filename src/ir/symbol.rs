use crate::ir::{ConsRef, DeclRef, Sign};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    decl_signs: HashMap<Vec<Sign>, Vec<DeclRef>>,
    cons_signs: HashMap<Vec<Sign>, Vec<ConsRef>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            decl_signs: HashMap::new(),
            cons_signs: HashMap::new(),
        }
    }

    pub fn iter_decl<'me>(&'me self) -> impl Iterator<Item = DeclRef> + 'me {
        self.decl_signs.values().flatten().copied()
    }

    pub fn iter_cons<'me>(&'me self) -> impl Iterator<Item = ConsRef> + 'me {
        self.cons_signs.values().flatten().copied()
    }

    pub fn lookup_decl(&self, sign: &[Sign]) -> &[DeclRef] {
        self.decl_signs.get(sign).map(AsRef::as_ref).unwrap_or(&[])
    }

    pub fn lookup_cons(&self, sign: &[Sign]) -> &[ConsRef] {
        self.cons_signs.get(sign).map(AsRef::as_ref).unwrap_or(&[])
    }

    pub fn new_decl(&mut self, decl_ref: DeclRef, sign: Vec<Sign>) {
        use std::collections::hash_map::Entry;
        match self.decl_signs.entry(sign) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(decl_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(decl_ref),
        };
    }

    pub fn new_cons(&mut self, cons_ref: ConsRef, sign: Vec<Sign>) {
        use std::collections::hash_map::Entry;
        match self.cons_signs.entry(sign) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(cons_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(cons_ref),
        };
    }
}
