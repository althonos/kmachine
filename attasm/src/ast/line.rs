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
