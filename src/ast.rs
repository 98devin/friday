use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

#[derive(Debug, Clone)]
pub struct OwnedToken(pub usize, pub String);

impl From<parser::Token<'_>> for OwnedToken {
    fn from(tok: parser::Token) -> OwnedToken {
        OwnedToken(tok.0, tok.1.to_owned())
    }
}

impl std::fmt::Display for OwnedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        parser::Token(self.0, &self.1).fmt(f)
    }
}
pub type Slice<'ctx, T> = &'ctx [T];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ident<'ctx>(pub &'ctx str);

#[derive(Debug, Copy, Clone)]
pub enum Atom<'ctx, T> {
    Hole,
    Unit,
    Number(f64),
    Ident(Ident<'ctx>),
    String(&'ctx str),
    Nested(&'ctx T),
}

#[derive(Debug, Copy, Clone)]
pub enum Expr<'ctx> {
    Flat(&'ctx [Atom<'ctx, Expr<'ctx>>]),
    Func(&'ctx Patn<'ctx>, &'ctx Expr<'ctx>),
    Match(&'ctx Expr<'ctx>, &'ctx [(Patn<'ctx>, Expr<'ctx>)]),
    Scoped(&'ctx [Decl<'ctx>], &'ctx Expr<'ctx>),
}

#[derive(Debug, Copy, Clone)]
pub enum Decl<'ctx> {
    Let(&'ctx Patn<'ctx>, &'ctx Expr<'ctx>),
    Def(&'ctx [Sign<'ctx>], &'ctx Expr<'ctx>),
    Con(&'ctx [Sign<'ctx>]),
    Mod(Ident<'ctx>, &'ctx Modl<'ctx>),
    Use(&'ctx Modl<'ctx>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModlPath<'ctx> {
    pub path: &'ctx [Ident<'ctx>],
    pub absolute: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Modl<'ctx> {
    ModExp(&'ctx [Decl<'ctx>]),
    Named(ModlPath<'ctx>),
}

#[derive(Debug, Copy, Clone)]
pub enum Patn<'ctx> {
    Flat(&'ctx [Atom<'ctx, Patn<'ctx>>]),
    Scoped(&'ctx [Decl<'ctx>], &'ctx Patn<'ctx>),
}

#[derive(Debug, Copy, Clone)]
pub enum Sign<'ctx> {
    Word(Ident<'ctx>),
    Patn(&'ctx Patn<'ctx>),
}

use crate::refs::*;
use crate::storage::*;

#[derive(Debug, Clone)]
pub struct AstStorage<'ctx> {
    pub expr: VecStorage<self::Expr<'ctx>, ExprRef>,
    pub patn: VecStorage<self::Patn<'ctx>, PatnRef>,
    pub decl: VecStorage<self::Decl<'ctx>, DeclRef>,
    pub cons: VecStorage<self::Decl<'ctx>, ConsRef>,
    pub modl: VecStorage<self::Modl<'ctx>, ModlRef>,
}

impl AstStorage<'_> {
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

use std::fmt::{self, Display, Formatter};

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Display for Atom<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Atom::Hole => write!(f, "_"),
            Atom::Unit => write!(f, "()"),
            Atom::Number(n) => write!(f, "{}", n),
            Atom::Ident(id) => write!(f, "{}", id),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Nested(t) => write!(f, "({})", t),
        }
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Flat(es) => {
                let (first, rest) = es.split_first().unwrap();
                write!(f, "{}", first)?;
                for exp in rest {
                    write!(f, " {}", exp)?;
                }
                Ok(())
            }

            Expr::Func(pat, exp) => write!(f, "fun {} = {}", pat, exp),

            Expr::Match(exp, cases) => {
                write!(f, "match {} ", exp)?;
                for (pat, body) in cases.iter() {
                    write!(f, "| {} = {} ", pat, body)?;
                }
                write!(f, "end")
            }

            Expr::Scoped(decls, exp) => {
                for decl in decls.iter() {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "in {}", exp)
            }
        }
    }
}

impl Display for Decl<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Decl::Let(pat, exp) => write!(f, "let {} = {}", pat, exp),
            Decl::Mod(id, modl) => write!(f, "mod {} = {}", id, modl),
            Decl::Use(modl) => write!(f, "use {}", modl),
            Decl::Def(sig, exp) => {
                write!(f, "def ")?;
                for sign in sig.iter() {
                    match sign {
                        Sign::Word(id) => write!(f, "{} ", id)?,
                        Sign::Patn(pat) => write!(f, "({}) ", pat)?,
                    }
                }
                write!(f, "= {}", exp)
            }
            Decl::Con(sig) => {
                write!(f, "con")?;
                for sign in sig.iter() {
                    write!(f, " {}", sign)?;
                }
                Ok(())
            }
        }
    }
}

impl Display for ModlPath<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.absolute {
            write!(f, ".")?;
        }

        let (last, init) = self.path.split_last().unwrap();

        for id in init {
            write!(f, "{}.", id)?;
        }

        write!(f, "{}", last)
    }
}

impl Display for Modl<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Modl::ModExp(decls) => {
                write!(f, "mod ")?;
                for decl in decls.iter() {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "end")
            }
            Modl::Named(path) => write!(f, "{}", path),
        }
    }
}

impl Display for Patn<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Patn::Flat(ps) => {
                let (first, rest) = ps.split_first().unwrap();
                write!(f, "{}", first)?;
                for pat in rest {
                    write!(f, " {}", pat)?;
                }
                Ok(())
            }
            Patn::Scoped(decls, pat) => {
                for decl in decls.iter() {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "in {}", pat)
            }
        }
    }
}

impl Display for Sign<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Sign::Word(id) => write!(f, "{}", id),
            Sign::Patn(pat) => write!(f, "{}", pat),
        }
    }
}
