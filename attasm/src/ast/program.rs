use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::iter::FromIterator;
use std::vec::IntoIter as VecIntoIter;
use std::slice::Iter as SliceIter;

use super::Line;

/// An assembler program written in AT&T syntax.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Program<'a> {
    lines: Vec<Line<'a>>
}

impl<'a> Program<'a> {
    pub fn lines(&self) -> &Vec<Line<'a>> {
        &self.lines
    }
}

impl<'a> From<Vec<Line<'a>>> for Program<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self {
            lines
        }
    }
}

impl<'a> FromIterator<Line<'a>> for Program<'a> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Line<'a>>,
    {
        Self {
            lines: iter.into_iter().collect(),
        }
    }
}

impl<'a> IntoIterator for Program<'a> {
    type Item = Line<'a>;
    type IntoIter = VecIntoIter<Line<'a>>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.lines.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b Program<'a> {
    type Item = &'b Line<'a>;
    type IntoIter = SliceIter<'b, Line<'a>>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        (&self.lines).into_iter()
    }
}

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut lines = self.lines().iter();
        while let Some(line) = lines.next() {
            line.fmt(f)?;
        }
        Ok(())
    }
}
