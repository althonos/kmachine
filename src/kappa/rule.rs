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
            rate,
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

    pub fn slot(&mut self, left: Agent, right: Agent) -> &mut Self {
        self.left.push(left);
        self.right.push(right);
        self
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut agents;

        // write rule name if any
        if let Some(ref name) = self.name {
            write!(f, "'{}'", name)?;
            f.write_char(if f.alternate() { '\n' } else { ' ' })?;
        }

        // write left slots
        agents = self.left.iter().peekable();
        while let Some(agent) = agents.next() {
            // Add indendation if pretty-printing
            if f.alternate() {
                f.write_str("    ")?;
            }
            // Write the agent in compact mode even if pretty-printing
            write!(f, "{}", agent)?;
            // Add separator if there are still some agents left
            if agents.peek().is_some() {
                f.write_str(if f.alternate() { ",\n" } else { ", " })?;
            }
        }

        // Add reaction arrow
        f.write_str(if f.alternate() { "\n->\n" } else { " -> " })?;

        // write right slots
        agents = self.right.iter().peekable();
        while let Some(agent) = agents.next() {
            // Add indendation if pretty-printing
            if f.alternate() {
                f.write_str("    ")?;
            }
            // Write the agent in compact mode even if pretty-printing
            write!(f, "{}", agent)?;
            // Add separator if there are still some agents left
            if agents.peek().is_some() {
                f.write_str(if f.alternate() { ",\n" } else { ", " })?;
            }
        }

        // write rate
        f.write_char(if f.alternate() { '\n' } else { ' ' })?;
        write!(f, "@ {}\n", self.rate)
    }
}
