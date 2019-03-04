use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use super::Label;
use super::Literal;
use super::Register;
use crate::parser::Parser;
use crate::parser::Rule;

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

impl<'a> TryFrom<&'a str> for Arg<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::arg, s).and_then(|mut pairs| {
            let pair = pairs.next().unwrap();
            check_complete!(pair, s);
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::register => Register::try_from(inner.as_str()).map(Arg::from),
                Rule::literal => Literal::try_from(inner.as_str()).map(Arg::from),
                Rule::label => Label::try_from(inner.as_str()).map(Arg::from),
                _ => unreachable!(),
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_label() {
        assert_eq!(Arg::try_from("label"), Ok(Arg::Label(Label::new("label"))));
    }

    #[test]
    fn parse_literal() {
        assert_eq!(Arg::try_from("$0xA"), Ok(Arg::Literal(Literal::Hex(0xA))));
    }

    #[test]
    fn parse_register() {
        assert_eq!(
            Arg::try_from("%rax"),
            Ok(Arg::Register(Register::new("rax")))
        );
    }
}
