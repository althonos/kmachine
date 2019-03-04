use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use crate::parser::Parser;
use crate::parser::Rule;
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

impl<'a> TryFrom<&'a str> for Line<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::singleline, s)
            .and_then(|mut pairs| {


                let pair = pairs.next().unwrap();
                println!("{:?}", pair);

                // check_complete!(pair, s);

                match pair.as_rule() {
                    Rule::labeldecl => {
                        let label = pair.into_inner().next().unwrap().as_str();
                        Label::try_from(label).map(|l| Line::LabelLine(l))
                    }
                    Rule::instruction => {
                        Instruction::try_from(pair.as_str()).map(|i| Line::OpLine(i))
                    }
                    x =>
                        unreachable!("{:?}", x),
                }
            })
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::Register;
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Line::try_from("label:"),
            Ok(Line::LabelLine(Label::new("label")))
        );

        assert_eq!(
            Line::try_from("nop\n"),
            Ok(Line::OpLine(Instruction::new("nop")))
        );

        let args = vec![Register::new("rax").into(), Register::new("rbx").into()];
        assert_eq!(
            Line::try_from("mov %rax, %rbx"),
            Ok(Line::OpLine(Instruction::with_args("mov", args)))
        );
    }

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
