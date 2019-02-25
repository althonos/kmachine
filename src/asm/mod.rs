pub mod att;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Register {
    pub name: String,
}

impl Register {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Label {
    pub name: String,
}

impl Label {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Inc(Register),
    Dec(Register),
    Jz(Register, Label),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Line {
    LabelLine(Label),
    OpLine(Op),
}

impl From<Label> for Line {
    fn from(l: Label) -> Self {
        Line::LabelLine(l)
    }
}

impl From<Op> for Line {
    fn from(op: Op) -> Self {
        Line::OpLine(op)
    }
}

pub trait AsmParser {
    fn parse_asm(s: &str) -> Vec<Line>;
}
