use derive_more::{From, Into};

use crate::ast;
use crate::ir::Storage;

use std::collections::HashMap;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into)]
pub struct Ident(usize);

#[derive(Debug, Clone)]
pub struct NameTable {
    name_to_id: HashMap<String, Ident>,
    id_to_name: HashMap<Ident, String>,
}

impl NameTable {
    pub fn new() -> NameTable {
        NameTable {
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
        }
    }

    pub fn get_ident(&self, name: &String) -> Option<Ident> {
        self.name_to_id.get(name).copied()
    }

    pub fn make_ident(&mut self, name: String) -> Ident {
        use std::collections::hash_map::Entry;

        let next_ix = self.name_to_id.len();
        match self.name_to_id.entry(name.clone()) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => {
                let id = *vacant.insert(Ident(next_ix));
                let _ = self.id_to_name.insert(id, name);
                id
            }
        }
    }
}

impl<'s> Storage<Ident> for &'s NameTable {
    type Stored = String;
    type StoredRef = &'s String;
    fn get(self, id: Ident) -> Option<Self::StoredRef> {
        self.id_to_name.get(&id)
    }
}
