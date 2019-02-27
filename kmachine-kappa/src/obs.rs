//!

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::expression::AlgebraicExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct Observable {
    name: String,
    value: AlgebraicExpression,
}

impl Observable {
    pub fn new<N, A>(name: N, value: A) -> Self
    where
        N: Into<String>,
        A: Into<AlgebraicExpression>,
    {
        Self {
            name: name.into(),
            value: value.into()
        }
    }

}

impl Display for Observable {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("%obs: '")
            .and(f.write_str(&self.name))
            .and(f.write_str("' "))
            .and(self.value.fmt(f))
            .and(f.write_char('\n'))
    }
}
