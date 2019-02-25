//!

mod agent;

use std::fmt::Display;
use std::fmt::Formatter;

pub use self::agent::Agent;
pub use self::agent::Site;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KappaProgram {
    vars: Vec<Var>,
    agents: Vec<Agent>,
    // rules: Vec<Rule>,
    // init: Vec<Init>,
}

impl KappaProgram {
    pub fn new() -> Self {
        KappaProgram {
            vars: Vec::new(),
            agents: Vec::new(),
        }
    }

    pub fn agent(&mut self, agent: Agent) -> &mut Self {
        self.agents.push(agent);
        self
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Var {
    pub name: String,
    pub value: u64,
}
