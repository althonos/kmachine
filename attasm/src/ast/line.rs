use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

use super::Label;
use super::Instruction;

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

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Line::LabelLine(l) => write!(f, "{}:\n", l),
            Line::OpLine(i) => write!(f, "\t{}\n", i),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::Register;
    use super::*;

    #[test]
    fn to_string() {
        let line: Line = Label::new("test").into();
        assert_eq!(&line.to_string(), "test:\n");

        let mut ins = Instruction::new("mov");
        ins.add_argument(Register::new("rax"));
        ins.add_argument(Register::new("rbx"));
        let line: Line = ins.into();
        assert_eq!(&line.to_string(), "\tmov\t%rax, %rbx\n");
    }

}
