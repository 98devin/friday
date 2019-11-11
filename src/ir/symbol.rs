
use std::collections::{HashMap};
use crate::ir::{Ident, Decl, Data};

#[derive(Debug, Clone)]
pub struct SymbolTable {
    decls: Vec<Decl>,
    datas: Vec<Data>,

    decl_signs: HashMap<Box<[Sign]>, Vec<DeclRef>>,
    data_signs: HashMap<Box<[Sign]>, Vec<DataRef>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Sign {
    Patn,
    Word(Ident),
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeclRef(usize);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DataRef(usize);

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            decls: Vec::new(),
            datas: Vec::new(),
            decl_signs: HashMap::new(),
            data_signs: HashMap::new(),
        }
    }

    pub fn get_decl(&self, decl_ref: DeclRef) -> &Decl {
        &self.decls[decl_ref.0]
    }

    pub fn get_data(&self, data_ref: DataRef) -> &Data {
        &self.datas[data_ref.0]
    }

    pub fn get_decl_mut(&mut self, decl_ref: DeclRef) -> &mut Decl {
        &mut self.decls[decl_ref.0]
    }

    pub fn get_data_mut(&mut self, data_ref: DataRef) -> &mut Data {
        &mut self.datas[data_ref.0]
    }

    pub fn lookup_decls(&self, sign: &[Sign]) -> &[DeclRef] {
        match self.decl_signs.get(sign) {
            Some(vec) => &vec,
            None      => &[],
        }
    }

    pub fn lookup_datas(&self, sign: &[Sign]) -> &[DataRef] {
        match self.data_signs.get(sign) {
            Some(vec) => &vec,
            None      => &[],
        }
    }

    pub fn add_decl(&mut self, decl: Decl, sign: &[Sign]) -> DeclRef {
        let decl_ref = DeclRef(self.decls.len());
        self.decls.push(decl);

        use std::collections::hash_map::Entry;
        match self.decl_signs.entry(sign.into()) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(decl_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(decl_ref),
        }

        decl_ref
    }

    pub fn add_data(&mut self, data: Data, sign: &[Sign]) -> DataRef {
        let data_ref = DataRef(self.datas.len());
        self.datas.push(data);

        use std::collections::hash_map::Entry;
        match self.data_signs.entry(sign.into()) {
            Entry::Occupied(mut occupied) => occupied.get_mut().push(data_ref),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()).push(data_ref),
        }

        data_ref
    }

}