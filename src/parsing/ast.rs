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

use std::fmt::{self, Display, Formatter};

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Display for Atom<T>
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

impl Display for Expr {
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
                for (pat, body) in cases {
                    write!(f, "| {} = {} ", pat, body)?;
                }
                write!(f, "end")
            }

            Expr::Scoped(decls, exp) => {
                for decl in decls {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "in {}", exp)
            }
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Decl::Let(pat, exp) => write!(f, "let {} = {}", pat, exp),
            Decl::Mod(id, modl) => write!(f, "mod {} = {}", id, modl),
            Decl::Use(modl) => write!(f, "use {}", modl),
            Decl::Def(sig, exp) => {
                write!(f, "def ")?;
                for sign in sig {
                    match sign {
                        Sign::Word(id) => write!(f, "{} ", id)?,
                        Sign::Patn(pat) => write!(f, "({}) ", pat)?,
                    }
                }
                write!(f, "= {}", exp)
            }
            Decl::Con(sig) => {
                write!(f, "con")?;
                for sign in sig {
                    write!(f, " {}", sign)?;
                }
                Ok(())
            }
        }
    }
}

impl Display for ModlPath {
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

impl Display for Modl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Modl::ModExp(decls) => {
                write!(f, "mod ")?;
                for decl in decls {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "end")
            }
            Modl::Named(path) => write!(f, "{}", path),
        }
    }
}

impl Display for Patn {
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
                for decl in decls {
                    write!(f, "{} ", decl)?;
                }
                write!(f, "in {}", pat)
            }
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Sign::Word(id) => write!(f, "{}", id),
            Sign::Patn(pat) => write!(f, "{}", pat),
        }
    }
}
