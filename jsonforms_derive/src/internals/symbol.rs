use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const JSONFORMS: Symbol = Symbol("jsonforms");
pub const SKIP: Symbol = Symbol("Skip");
pub const VLAYOUT: Symbol = Symbol("VerticalLayout");
pub const HLAYOUT: Symbol = Symbol("HorizontalLayout");
pub const ELAYOUT: Symbol = Symbol("EndLayout");
pub const SCHEMA: Symbol = Symbol("schema");
pub const UISCHEMA: Symbol = Symbol("uischema");
pub const DEBUG: Symbol = Symbol("debug");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}