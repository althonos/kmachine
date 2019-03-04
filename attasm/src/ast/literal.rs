use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use crate::parser::Parser;
use crate::parser::Rule;

/// A literal value, e.g. `$0x1A`, `$12` or `$address`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Literal<'a> {
    Dec(usize),
    Hex(usize),
    Oct(usize),
    Address(Cow<'a, str>),
}

impl<'a> Display for Literal<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Literal::Dec(n) => write!(f, "${}", n),
            Literal::Hex(n) => write!(f, "$0x{:X}", n),
            Literal::Oct(n) => write!(f, "$0o{:o}", n),
            Literal::Address(n) => write!(f, "${}", n),
        }
    }
}

impl<'a> TryFrom<&'a str> for Literal<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::literal, s).and_then(|mut pairs| {
            let pair = pairs.next().unwrap();
            check_complete!(pair, s);
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::number => {
                    let n = usize::from_str_radix(inner.as_str(), 10);
                    Ok(Literal::Dec(n.unwrap()))
                }
                Rule::octnumber => {
                    let n = usize::from_str_radix(inner.as_str(), 8);
                    Ok(Literal::Oct(n.unwrap()))
                }
                Rule::hexnumber => {
                    let n = usize::from_str_radix(inner.as_str(), 16);
                    Ok(Literal::Hex(n.unwrap()))
                }
                Rule::ident => {
                    let a = inner.as_str().into();
                    Ok(Literal::Address(a))
                }
                _ => unreachable!(),
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_hex() {
        assert_eq!(Literal::try_from("$0x1A"), Ok(Literal::Hex(0x1A)));
    }

    #[test]
    fn parse_dec() {
        assert_eq!(Literal::try_from("$18"), Ok(Literal::Dec(18)));
    }

    #[test]
    fn parse_oct() {
        assert_eq!(Literal::try_from("$0o74"), Ok(Literal::Oct(0o74)));
    }

    #[test]
    fn parse_address() {
        assert_eq!(
            Literal::try_from("$LCA"),
            Ok(Literal::Address("LCA".into()))
        );
    }
}
