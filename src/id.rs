use derive_more::{From, Into};

use crate::storage::*;

use std::collections::HashMap;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct Ident(usize);

#[derive(Debug, Clone)]
pub struct NameTable<'ctx> {
    name_to_id: HashMap<&'ctx str, Ident>,
    id_to_name: HashMap<Ident, &'ctx str>,
}

impl<'ctx> NameTable<'ctx> {
    pub fn new() -> Self {
        NameTable {
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
        }
    }

    pub fn get_ident(&self, name: &str) -> Option<Ident> {
        self.name_to_id.get(name).copied()
    }

    pub fn make_ident(&mut self, name: &'ctx str) -> Ident {
        use std::collections::hash_map::Entry;

        let next_ix = self.name_to_id.len();
        match self.name_to_id.entry(name) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => {
                let id = *vacant.insert(Ident(next_ix));
                let _ = self.id_to_name.insert(id, name);
                id
            }
        }
    }
}

impl<'r> Storage<'r, Ident> for NameTable<'_> {
    type Stored = str;
    type StoredRef = &'r str;
    fn get(&'r self, id: Ident) -> Option<Self::StoredRef> {
        self.id_to_name.get(&id).map(|s| *s)
    }
}
