use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::agent::Agent;

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    name: Option<String>,
    left: Vec<Agent>,
    right: Vec<Agent>,
    rate: f64,
}

impl Rule {
    pub fn new(rate: f64) -> Self {
        Rule {
            name: None,
            left: Vec::new(),
            right: Vec::new(),
            rate
        }
    }

    pub fn with_name<N>(name: N, rate: f64) -> Self
    where
        N: Into<String>,
    {
        let mut rule = Self::new(rate);
        rule.name = Some(name.into());
        rule
    }

    pub fn agent(&mut self, left: Agent, right: Agent) -> &mut Self {
        self.left.push(left);
        self.right.push(right);
        self
    }
}


impl Display for Rule {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {

        let mut agents;

        if let Some(ref name) = self.name {
            write!(f, "'{}' ", name)?;
        }

        agents = self.left.iter().peekable();
        while let Some(agent) = agents.next() {
            agent.fmt(f)?;
            if agents.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        f.write_str(" -> ")?;

        agents = self.right.iter().peekable();
        while let Some(agent) = agents.next() {
            agent.fmt(f)?;
            if agents.peek().is_some() {
                f.write_str(", ")?;
            }
        }

        write!(f, " @ {}\n", self.rate)



    }


}
