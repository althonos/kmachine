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

/// A literal in the program, e.g. `$0x8`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Literal {
    value: usize,
}

impl Literal {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

/// An operand with its arguments, e.g. `jnz %rax, start`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction<'a> {
    op: Cow<'a, str>,
    args: Vec<Arg<'a>>,
}

impl<'a> Instruction<'a> {
    pub fn new<O>(op: O) -> Self
    where
        O: Into<Cow<'a, str>>,
    {
        Self {
            op: op.into(),
            args: Vec::new(),
        }
    }

    pub fn with_args<O, A>(op: O, args: A) -> Self
    where
        O: Into<Cow<'a, str>>,
        A: IntoIterator<Item = Arg<'a>>,
    {
        let mut ins = Self::new(op);
        for arg in args.into_iter() {
            ins.argument(arg);
        }
        ins
    }

    pub fn argument<A>(&mut self, arg: A) -> &mut Self
    where
        A: Into<Arg<'a>>,
    {
        self.args.push(arg.into());
        self
    }

    pub fn op(&self) -> &str {
        &self.op
    }

    pub fn arguments(&self) -> &Vec<Arg<'a>> {
        &self.args
    }
}

/// An argument to an instruction, e.g. `%rax` or `$1`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Arg<'a> {
    Register(Register<'a>),
    Label(Label<'a>),
    Literal(Literal),
}

impl<'a> From<Register<'a>> for Arg<'a> {
    fn from(r: Register<'a>) -> Self {
        Arg::Register(r)
    }
}

impl<'a> From<Label<'a>> for Arg<'a> {
    fn from(l: Label<'a>) -> Self {
        Arg::Label(l)
    }
}

impl<'a> From<Literal> for Arg<'a> {
    fn from(l: Literal) -> Self {
        Arg::Literal(l)
    }
}

/// A program line.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Line<'a> {
    LabelLine(Label<'a>),
    OpLine(Instruction<'a>),
}

impl<'a> From<Label<'a>> for Line<'a> {
    fn from(l: Label<'a>) -> Self {
        Line::LabelLine(l)
    }
}

impl<'a> From<Instruction<'a>> for Line<'a> {
    fn from(ins: Instruction<'a>) -> Self {
        Line::OpLine(ins)
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
        self.lines()
            .into_iter()
            .flat_map(|ref line| match line {
                Line::LabelLine(_) => None,
                Line::OpLine(ins) => match ins.op() {
                    "clr" | "dec" | "inc" | "jz" => match ins.arguments().first() {
                        Some(Arg::Register(r)) => Some(r),
                        _ => None,
                    },
                    _ => None,
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
        self.lines()
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
