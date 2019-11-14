use lalrpop_util::lalrpop_mod;

pub mod ast;
lalrpop_mod!(pub parser, "/parsing/parser.rs");

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
