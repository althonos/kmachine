use std::iter::FromIterator;

use super::Line;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Program<'a> {
    lines: Vec<Line<'a>>
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
