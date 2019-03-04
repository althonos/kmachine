use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use crate::parser::Parser;
use crate::parser::Rule;

/// A label somewhere in the program.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label<'a> {
    name: Cow<'a, str>,
}

impl<'a> Label<'a> {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Label { name: name.into() }
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

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.name.fmt(f)
    }
}

impl<'a> TryFrom<&'a str> for Label<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::label, s).and_then(|mut pairs| {
            let pair = pairs.next().unwrap();
            check_complete!(pair, s);
            Ok(Label::new(pair.as_str()))
        })
    }
}
