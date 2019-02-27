//!

#[macro_use]
mod macros;

mod agent;
mod init;
mod link;
mod rule;
mod site;
mod pattern;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

pub use self::agent::Agent;
pub use self::agent::AgentDecl;
pub use self::init::Init;
pub use self::link::Link;
pub use self::rule::Rule;
pub use self::site::Site;

#[derive(Clone, Debug, PartialEq)]
pub struct KappaProgram {
    agents: Vec<AgentDecl>,
    rules: Vec<Rule>,
    inits: Vec<Init>,
}

impl KappaProgram {
    pub fn new() -> Self {
        KappaProgram {
            agents: Vec::new(),
            rules: Vec::new(),
            inits: Vec::new(),
        }
    }

    pub fn agent<A>(&mut self, agent: A) -> &mut Self
    where
        A: Into<AgentDecl>,
    {
        self.agents.push(agent.into());
        self
    }

    pub fn rule<R>(&mut self, rule: R) -> &mut Self
    where
        R: Into<Rule>,
    {
        self.rules.push(rule.into());
        self
    }

    pub fn init<I>(&mut self, init: I) -> &mut Self
    where
        I: Into<Init>,
    {
        self.inits.push(init.into());
        self
    }
}

impl Display for KappaProgram {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // write agents
        for agent in self.agents.iter() {
            agent.fmt(f)?;
        }

        if f.alternate() {
            f.write_char('\n')?;
        }

        // write rules
        for rule in self.rules.iter() {
            rule.fmt(f)?;
        }

        if f.alternate() {
            f.write_char('\n')?;
        }

        // write inits
        for init in self.inits.iter() {
            init.fmt(f)?;
        }

        Ok(())
    }
}
