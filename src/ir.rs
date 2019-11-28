pub mod symbol;

use crate::storage::*;

use crate::ast;
use crate::ctx::Context;
use crate::id::Ident;
use crate::refs::*;
use crate::symbol::SymbolTable;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Unit,
    Number(f64),
    String(String),
}

// TODO: this might not be necessary.
impl Eq for Literal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Hole,
    Literal(Literal),
    Var(DeclRef),
    Data(ConsRef, Vec<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Func(Box<Patn>, Box<Expr>),
    Match(Box<Expr>, Vec<(Patn, Expr)>),
    Scoped(ModlRef, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patn {
    Empty,
    Literal(Literal),
    Binding(Vec<Ident>),
    Data(ConsRef, Vec<Patn>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    pub sig: Vec<Sign<PatnRef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cons {
    pub sig: Vec<Sign>,
}

#[derive(Debug, Clone)]
pub enum Modl {
    Record(ModlRecord),
    Alias(ModlAlias),
}

#[derive(Debug, Clone)]
pub struct ModlRecord {
    pub name: String,
    pub scope: Vec<ModlRef>,
    pub decls: Vec<DeclRef>,
    pub cons: Vec<ConsRef>,
    pub symbols: SymbolTable,
    pub children: HashMap<Ident, ModlRef>,
}

#[derive(Debug, Clone)]
pub struct ModlAlias {
    pub name: String,
    pub scope: ModlRef,
    pub aliased: Option<ModlRef>,
    pub path: Vec<Ident>,
}

impl Modl {
    pub fn new(name: String) -> Self {
        Modl::Record(ModlRecord {
            name,
            scope: Vec::new(),
            decls: Vec::new(),
            cons: Vec::new(),
            symbols: SymbolTable::new(),
            children: HashMap::new(),
        })
    }

    pub fn as_record(&mut self) -> &mut ModlRecord {
        match self {
            Modl::Record(ref mut record) => record,
            Modl::Alias(_) => panic!("Module was not a record!"),
        }
    }

    pub fn as_alias(&mut self) -> &mut ModlAlias {
        match self {
            Modl::Alias(ref mut alias) => alias,
            Modl::Record(_) => panic!("Module was not an alias!"),
        }
    }

    pub fn new_alias<'ctx>(name: String, scope: ModlRef) -> Self {
        Modl::Alias(ModlAlias {
            name,
            scope,
            aliased: None,
            path: Vec::new(),
        })
    }

    pub fn name(&self) -> &String {
        match self {
            Modl::Alias(alias) => &alias.name,
            Modl::Record(record) => &record.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sign<T = ()> {
    Patn(T),
    Word(Ident),
}

impl<T> Sign<T> {
    pub fn forget(self) -> Sign<()> {
        match self {
            Sign::Patn(_) => Sign::Patn(()),
            Sign::Word(id) => Sign::Word(id),
        }
    }
}

impl<'ctx> ast::Sign<'ctx> {
    pub fn from_ast(self, ctx: &Context<'ctx>) -> Sign<ast::Patn<'ctx>> {
        let mut names = ctx.names.borrow_mut();
        match self {
            ast::Sign::Word(ast::Ident(id)) => Sign::Word(names.make_ident(id)),
            ast::Sign::Patn(pat) => Sign::Patn(*pat),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrStorage {
    pub expr: VecStorage<self::Expr, ExprRef>,
    pub patn: VecStorage<self::Patn, PatnRef>,
    pub decl: VecStorage<self::Decl, DeclRef>,
    pub cons: VecStorage<self::Cons, ConsRef>,
    pub modl: VecStorage<self::Modl, ModlRef>,
}

impl IrStorage {
    pub fn new() -> Self {
        Self {
            expr: VecStorage::new(),
            patn: VecStorage::new(),
            decl: VecStorage::new(),
            cons: VecStorage::new(),
            modl: VecStorage::new(),
        }
    }
}
