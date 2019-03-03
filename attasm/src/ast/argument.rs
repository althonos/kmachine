use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

use super::Register;
use super::Label;
use super::Literal;

/// An argument to an instruction, e.g. `%rax` or `$1`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Arg<'a> {
    Register(Register<'a>),
    Label(Label<'a>),
    Literal(Literal<'a>),
}

impl<'a> From<Register<'a>> for Arg<'a> {
    fn from(r: Register<'a>) -> Self {
        Arg::Register(r)
    }
}

impl<'a> From<Label<'a>> for Arg<'a> {
    fn from(l: Label<'a>) -> Self {
        Arg::Label(l)
    }
}

impl<'a> From<Literal<'a>> for Arg<'a> {
    fn from(l: Literal<'a>) -> Self {
        Arg::Literal(l)
    }
}

impl<'a> Display for Arg<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Arg::Register(r) => r.fmt(f),
            Arg::Label(l) => l.fmt(f),
            Arg::Literal(l) => l.fmt(f),
        }
    }
}
