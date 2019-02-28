//!

extern crate indexmap;

#[macro_export]
mod macros;

mod agent;
mod expression;
mod init;
mod link;
mod obs;
mod pattern;
mod rule;
mod site;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

pub use self::agent::Agent;
pub use self::agent::AgentDecl;
pub use self::expression::AlgebraicExpression;
pub use self::init::Init;
pub use self::link::Link;
pub use self::obs::Observable;
pub use self::pattern::Pattern;
pub use self::rule::Rule;
pub use self::site::Site;

#[derive(Clone, Debug, PartialEq)]
pub struct KappaProgram {
    agents: Vec<AgentDecl>,
    rules: Vec<Rule>,
    inits: Vec<Init>,
    obs: Vec<Observable>,
}

impl KappaProgram {
    pub fn new() -> Self {
        KappaProgram {
            agents: Vec::new(),
            rules: Vec::new(),
            inits: Vec::new(),
            obs: Vec::new(),
        }
    }

    pub fn agent<A>(&mut self, agent: A) -> &mut Self
    where
        A: Into<AgentDecl>,
    {
        self.agents.push(agent.into());
        self
    }

    pub fn init<I>(&mut self, init: I) -> &mut Self
    where
        I: Into<Init>,
    {
        self.inits.push(init.into());
        self
    }

    pub fn observable<O>(&mut self, observable: O) -> &mut Self
    where
        O: Into<Observable>,
    {
        self.obs.push(observable.into());
        self
    }

    pub fn rule<R>(&mut self, rule: R) -> &mut Self
    where
        R: Into<Rule>,
    {
        self.rules.push(rule.into());
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

        if f.alternate() {
            f.write_char('\n')?;
        }

        // write observables
        for obs in self.obs.iter() {
            obs.fmt(f)?;
        }

        Ok(())
    }
}
