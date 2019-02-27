//!

use std::ops::Deref;

use super::Agent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    agents: Vec<Agent>,
}

impl Pattern {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn agent<A>(&mut self, agent: A) -> &mut Self
    where
        A: Into<Agent>,
    {
        self.agents.push(agent.into());
        self
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            agents: Vec::new()
        }
    }
}

impl Deref for Pattern {
    type Target = Vec<Agent>;
    fn deref(&self) -> &Vec<Agent> {
        &self.agents
    }
}
