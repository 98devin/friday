use crate::ast;
use crate::ir::{Context, Ident, ModuleRef, Storage, WithContext};

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
    Var(u64),
    Apply(Box<Expr>, Box<Expr>),
    Func(Box<Patn>, Box<Expr>),
    Match(Box<Expr>, Vec<(Patn, Expr)>),
    Scoped(ModuleRef, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Patn {
    Empty,
    Literal(Literal),
    Binding(Vec<Ident>),
    Data(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    pub sig: Vec<Sign<Patn>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cons {
    pub sig: Vec<Sign>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

impl Context {
    pub fn sign_from_ast(&mut self, ast_sign: &ast::Sign) -> Sign<Box<ast::Patn>> {
        match ast_sign {
            ast::Sign::Patn(pat) => Sign::Patn(pat.clone()),
            ast::Sign::Word(id) => Sign::Word(self.idents_mut().make_ident(id.0.clone())),
        }
    }
}

impl<'ctx, T> std::fmt::Display for WithContext<'ctx, &Sign<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.val {
            Sign::Patn(_) => write!(f, "_"),
            Sign::Word(id) => write!(f, "{}", self.idents().get(*id).unwrap()),
        }
    }
}

impl<'ctx, T> std::fmt::Display for WithContext<'ctx, &[Sign<T>]> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.val.is_empty() {
            return Ok(());
        }
        let (init, rest) = self.val.split_first().unwrap();
        write!(f, "{}", self.wrap(init))?;
        for sign in rest {
            write!(f, " {}", self.wrap(sign))?;
        }
        Ok(())
    }
}

impl Context {}
