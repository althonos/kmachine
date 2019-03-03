use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use crate::parser::Parser;
use crate::parser::Rule;

/// A register, e.g. `%rax`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Register<'a> {
    name: Cow<'a, str>,
}

impl<'a> Register<'a> {

    pub fn new<N>(name: N) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<N>(&mut self, name: N)
    where
        N: Into<Cow<'a, str>>,
    {
        self.name = name.into();
    }

}

impl<'a> Display for Register<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "%{}", &self.name)
    }
}

impl<'a> TryFrom<&'a str> for Register<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::register, s)
            .and_then(|mut pairs| {
                let pair = pairs.next().unwrap();
                check_complete!(pair, s);
                Ok(Self::new(pair.into_inner().as_str()))
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_string() {
        let r = Register::new("rax");
        assert_eq!(&r.to_string(), "%rax");
    }

    #[test]
    fn parse() {
        let r = Register::try_from("%rax");
        assert_eq!(r, Ok(Register::new("rax")));
        assert!(Register::try_from("%rax,").is_err());
        assert!(Register::try_from("%12").is_err());
    }

}
