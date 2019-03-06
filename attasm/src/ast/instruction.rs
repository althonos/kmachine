use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::error::Error as PestError;
use pest::Parser as PestParser;

use super::Arg;
use crate::parser::Parser;
use crate::parser::Rule;

/// A mnemonic with its arguments, e.g. `jnz %rax, start`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instruction<'a> {
    mnemonic: Cow<'a, str>,
    args: Vec<Arg<'a>>,
}

impl<'a> Instruction<'a> {
    pub fn new<M>(mnemonic: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            mnemonic: mnemonic.into(),
            args: Vec::new(),
        }
    }

    pub fn with_arguments<M, A>(mnemonic: M, args: A) -> Self
    where
        M: Into<Cow<'a, str>>,
        A: IntoIterator<Item = Arg<'a>>,
    {
        let mut ins = Self::new(mnemonic);
        ins.args = args.into_iter().collect();
        ins
    }

    pub fn add_argument<A>(&mut self, arg: A) -> &mut Self
    where
        A: Into<Arg<'a>>,
    {
        self.args.push(arg.into());
        self
    }

    pub fn mnemonic(&self) -> &str {
        &self.mnemonic
    }

    pub fn set_mnemonic<M>(&mut self, mnemonic: M)
    where
        M: Into<Cow<'a, str>>,
    {
        self.mnemonic = mnemonic.into();
    }

    pub fn arguments(&self) -> &Vec<Arg<'a>> {
        &self.args
    }
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(&self.mnemonic)?;
        let mut args = self.args.iter().peekable();
        if args.peek().is_some() {
            f.write_char('\t')?;
        }
        while let Some(arg) = args.next() {
            arg.fmt(f)?;
            if args.peek().is_some() {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = PestError<Rule>;
    fn try_from(s: &'a str) -> Result<Self, PestError<Rule>> {
        Parser::parse(Rule::instruction, s).and_then(|mut pairs| {
            let pair = pairs.next().unwrap();
            check_complete!(pair, s);
            let mut inner = pair.into_inner();

            let op = inner.next().unwrap().as_str();

            let mut args = Vec::new();
            for pair in inner {
                args.push(Arg::try_from(pair.as_str())?);
            }

            Ok(Self::with_arguments(op, args))
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::Register;

    #[test]
    fn parse() {
        let mut ins = Instruction::new("mov");
        ins.add_argument(Register::new("rax"));
        ins.add_argument(Register::new("rbx"));
        assert_eq!(Instruction::try_from("mov %rax, %rbx"), Ok(ins));

        assert!(Instruction::try_from("mov;").is_err());
    }

    #[test]
    fn to_string() {
        let mut ins = Instruction::new("nop");
        assert_eq!(&ins.to_string(), "nop");

        ins.set_mnemonic("inc");
        ins.add_argument(Register::new("rax"));
        assert_eq!(&ins.to_string(), "inc\t%rax");

        ins.set_mnemonic("mov");
        ins.add_argument(Register::new("rbx"));
        assert_eq!(&ins.to_string(), "mov\t%rax, %rbx");
    }

}
