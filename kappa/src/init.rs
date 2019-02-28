use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::agent::Agent;
use super::pattern::Pattern;

/// A Kappa initial mixture declaration, e.g. `%init: 1 A()`.
#[derive(Clone, Debug, PartialEq)]
pub struct Init {
    count: usize,
    mixture: Pattern,
}

impl Init {
    /// Create a new initial declaration.
    pub fn new(count: usize) -> Self {
        Self {
            count,
            mixture: Pattern::new(),
        }
    }

    /// Create a new initial declaration with a single agent.
    pub fn with_agent<A>(count: usize, agent: A) -> Self
    where
        A: Into<Agent>,
    {
        let mut init = Self::new(count);
        init.agent(agent);
        init
    }

    /// Add an agent to the initial declaration.
    pub fn agent<A>(&mut self, agent: A) -> &mut Self
    where
        A: Into<Agent>,
    {
        self.mixture.agent(agent);
        self
    }
}

impl Display for Init {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "%init: {} ", self.count)?;

        let mut agents = self.mixture.iter().peekable();
        while let Some(agent) = agents.next() {
            agent.fmt(f)?;
            if agents.peek().is_some() {
                f.write_char(',')?;
                f.write_char(if f.alternate() { '\n' } else { ' ' })?;
            }
        }

        f.write_char('\n')
    }
}
