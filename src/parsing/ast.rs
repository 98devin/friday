#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum Atom<T> {
    Hole,
    Unit,
    Number(f64),
    Ident(Ident),
    String(String),
    Nested(Box<T>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Flat(Vec<Atom<Expr>>),
    Func(Box<Patn>, Box<Expr>),
    Match(Box<Expr>, Vec<(Patn, Expr)>),
    Scoped(Vec<Decl>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Decl {
    Let(Box<Patn>, Box<Expr>),
    Def(Vec<Sign>, Box<Expr>),
    Con(Vec<Sign>),
    Mod(Ident, Box<Modl>),
    Use(Box<Modl>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModlPath {
    pub path: Vec<Ident>,
    pub absolute: bool,
}

#[derive(Debug, Clone)]
pub enum Modl {
    ModExp(Vec<Decl>),
    Named(ModlPath),
}

#[derive(Debug, Clone)]
pub enum Patn {
    Flat(Vec<Atom<Patn>>),
    Scoped(Vec<Decl>, Box<Patn>),
}

#[derive(Debug, Clone)]
pub enum Sign {
    Word(Ident),
    Patn(Box<Patn>),
}
