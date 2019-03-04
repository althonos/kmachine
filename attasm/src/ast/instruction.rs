use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::Arg;

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

    pub fn with_args<M, A>(mnemonic: M, args: A) -> Self
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

#[cfg(test)]
mod tests {

    use crate::ast::Register;
    use super::*;

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
