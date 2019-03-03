use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum State {
    Unknown,
    Known(String),
}

impl<S> From<S> for State
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        State::Known(s.into())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            State::Unknown => f.write_char('#'),
            State::Known(name) => f.write_str(&name),
        }
    }
}
