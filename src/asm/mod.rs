//! Abstract syntax structures for counter machine programs.

pub mod att;

use std::borrow::Cow;
use std::borrow::ToOwned;
use std::iter::FromIterator;

use indexmap::IndexSet;

/// A register, e.g. `%rax`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Register<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> Register<'a> {
    pub fn new<I>(name: I) -> Self
    where
        I: Into<Cow<'a, str>>,
    {
        Self { name: name.into() }
    }
}

/// A label in the program, e.g. `start:`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> Label<'a> {
    pub fn new<I>(name: I) -> Self
    where
        I: Into<Cow<'a, str>>,
    {
        Self { name: name.into() }
    }
}

/// An operand with its arguments, e.g. `jnz %rax, start`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op<'a> {
    Clr(Register<'a>),
    Dec(Register<'a>),
    Inc(Register<'a>),
    Jz(Register<'a>, Label<'a>),
}

/// A program line.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Line<'a> {
    LabelLine(Label<'a>),
    OpLine(Op<'a>),
}

impl<'a> From<Label<'a>> for Line<'a> {
    fn from(l: Label<'a>) -> Self {
        Line::LabelLine(l)
    }
}

impl<'a> From<Op<'a>> for Line<'a> {
    fn from(op: Op<'a>) -> Self {
        Line::OpLine(op)
    }
}

/// The abstract syntax tree of an abstract assembler program.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AsmProgram<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> AsmProgram<'a> {

    /// Create a new empty program.
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// Get the set of all registers *used* in the program.
    pub fn registers(&self) -> IndexSet<&Register<'a>> {
        self
            .lines()
            .into_iter()
            .flat_map(|ref line| match line {
                Line::LabelLine(_) => None,
                Line::OpLine(op) => match op {
                    Op::Clr(r) => Some(r),
                    Op::Inc(r) => Some(r),
                    Op::Dec(r) => Some(r),
                    Op::Jz(r, _) => Some(r),
                },
            })
            .collect()
    }

    /// Get the set of all labels *declared* in the program.
    ///
    /// This does not include labels that are used as arguments to jumping
    /// instructions such as `jz`.
    ///
    /// ## Example
    /// ```rust
    /// # use kappamachine::asm::{AttParser, AsmProgram, Label};
    /// let program: AsmProgram = AttParser.parse_asm(
    ///     """
    ///     label1:
    ///         inc %rax
    ///         jz  %rax, label2
    ///     """
    /// );
    ///
    /// assert_eq!(program.labels(), indexset!{Label::new("label1")}));
    /// ```
    pub fn labels(&self) -> IndexSet<&Label<'a>> {
        self
            .lines()
            .into_iter()
            .flat_map(|ref line| match line {
                Line::LabelLine(l) => Some(l),
                _ => None,
            })
            .collect()
    }

    pub fn lines(&self) -> &Vec<Line<'a>> {
        &self.lines
    }

    pub fn lines_mut(&mut self) -> &mut Vec<Line<'a>> {
        &mut self.lines
    }
}

impl<'a> FromIterator<Line<'a>> for AsmProgram<'a> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Line<'a>>,
    {
        Self {
            lines: iter.into_iter().collect(),
        }
    }
}

/// Parse a string into the corresponding abstract syntax tree.
pub trait AsmParser {
    fn parse_asm<'a>(s: &'a str) -> AsmProgram<'a>;
}
