use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

/// A label somewhere in the program.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label<'a> {
    name: Cow<'a, str>
}

impl<'a> Label<'a> {

    pub fn new<N>(name: N) -> Self
    where
        N: Into<Cow<'a, str>>
    {
        Label {
            name: name.into()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<N>(&mut self, name: N)
    where
        N: Into<Cow<'a, str>>
    {
        self.name = name.into();
    }
}

impl<'a> Display for Label<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.name.fmt(f)
    }
}
