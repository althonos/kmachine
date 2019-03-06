use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::iter::FromIterator;
use std::slice::Iter as SliceIter;
use std::vec::IntoIter as VecIntoIter;

use indexmap::IndexSet;
use pest::error::Error as PestError;
use pest::Parser as PestParser;

use super::Arg;
use super::Instruction;
use super::Label;
use super::Line;
use super::Register;
use crate::parser::Parser;
use crate::parser::Rule;

/// An assembler program written in AT&T syntax.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Program<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Program<'a> {
    pub fn lines(&self) -> &Vec<Line<'a>> {
        &self.lines
    }

    pub fn lines_mut(&mut self) -> &mut Vec<Line<'a>> {
        &mut self.lines
    }

    /// Get the set of all labels *declared* in the program.
    ///
    /// This does not include labels that are used as arguments to jumping
    /// instructions such as `jz`.
    ///
    /// ## Example
    /// ```rust
    /// # #[macro_use]
    /// # extern crate indexmap;
    /// # use asm::{AttParser, AsmParser, AsmProgram, Label};
    /// # pub fn main() {
    /// let program: AsmProgram = AttParser::parse_asm(
    ///     "
    ///     label1:
    ///         inc %rax
    ///         jz  %rax, label2
    ///     "
    /// );
    ///
    /// let label = Label::new("label1");
    /// assert_eq!(program.labels(), indexset!{&label});
    /// # }
    /// ```
    pub fn labels(&self) -> IndexSet<&Label<'a>> {
        self.lines()
            .into_iter()
            .flat_map(|ref line| match line {
                Line::LabelLine(l) => Some(l),
                _ => None,
            })
            .collect()
    }

    /// Get the set of all registers *used* in the program.
    pub fn registers(&self) -> IndexSet<&Register<'a>> {
        self.lines()
            .into_iter()
            .flat_map(|ref line| match line {
                Line::LabelLine(_) => None,
                Line::OpLine(ins) => Some(ins.arguments().iter().flat_map(|arg| match arg {
                    Arg::Register(r) => Some(r),
                    _ => None,
                })),
            })
            .flatten()
            .collect()
    }
}

impl<'a> From<Vec<Line<'a>>> for Program<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self { lines }
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

impl<'a> TryFrom<&'a str> for Program<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::program, s).and_then(|pairs| {
            let mut lines = Vec::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::EOI => (),
                    Rule::labeldecl => {
                        let label = pair.into_inner().next().unwrap();
                        lines.push(Label::try_from(label.as_str())?.into())
                    }
                    Rule::instruction => {
                        let ins = Instruction::try_from(pair.as_str())?;
                        lines.push(ins.into())
                    }
                    x => unreachable!("{:?}", x),
                }
            }
            Ok(Self::from(lines))
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::Literal;
    use crate::ast::Register;

    #[test]
    fn parse() {
        let program = Program::try_from("start:\n\tmov\t$10, %rax\ninc\t%rbx").unwrap();

        let mut lines = program.lines().iter();
        assert_eq!(lines.next(), Some(&Line::LabelLine(Label::new("start"))));
        assert_eq!(
            lines.next(),
            Some(&Line::OpLine(Instruction::with_args(
                "mov",
                vec![Literal::Dec(10).into(), Register::new("rax").into()]
            )))
        )
    }

}
