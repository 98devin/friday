
use crate::ir::{Ident, ModuleRef};

#[derive(Debug, Clone)]
pub enum Literal {
    Unit,
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Hole,
    Literal(Literal),
    Var(u64),
    Apply(Box<Expr>, Box<Expr>),
    Func(Box<Patn>, Box<Expr>),
    Match(Box<Expr>, Vec<(Patn, Expr)>),
    Scoped(ModuleRef, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Patn {
    Empty,
    Literal(Literal),
    Binding(Vec<Ident>),
    Data()
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
}