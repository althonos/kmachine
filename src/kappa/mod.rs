//!

#[macro_use]
mod macros;

mod agent;
mod rule;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

pub use self::agent::Agent;
pub use self::agent::Site;
pub use self::agent::Link;
pub use self::rule::Rule;

#[derive(Clone, Debug, PartialEq)]
pub struct KappaProgram {
    // vars: Vec<Var>,
    agents: Vec<Agent>,
    rules: Vec<Rule>,
    // init: Vec<Init>,
}

impl KappaProgram {
    pub fn new() -> Self {
        KappaProgram {
            // vars: Vec::new(),
            agents: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn agent(&mut self, agent: Agent) -> &mut Self {
        self.agents.push(agent);
        self
    }

    pub fn rule(&mut self, rule: Rule) -> &mut Self {
        self.rules.push(rule);
        self
    }
}

impl Display for KappaProgram {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        /*FIXME*/
        for agent in self.agents.iter() {
            write!(f, "%agent: {}\n", agent)?;
        }
        for rule in self.rules.iter() {
            write!(f, "{}", rule)?;
        }

        Ok(())
    }
}



// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Var {
//     pub name: String,
//     pub value: u64,
// }
