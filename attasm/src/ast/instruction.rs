use std::borrow::Cow;

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
