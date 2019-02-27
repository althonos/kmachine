use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use indexmap::IndexSet;

use super::Site;

/// A Kappa agent, e.g. `A()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Agent {
    pub name: String,
    pub sites: Vec<Site>,
}

impl Agent {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            sites: Vec::new(),
        }
    }

    pub fn site(&mut self, site: Site) -> &mut Self {
        self.sites.push(site);
        self
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}(", self.name)?;
        if f.alternate() {
            f.write_char('\n')?;
        }
        let sites = &mut self.sites.iter().peekable();
        while let Some(ref site) = sites.next() {
            if f.alternate() {
                write!(f, "    {}", site)?;
            } else {
                write!(f, "{}", site)?;
            }
            if sites.peek().is_some() {
                f.write_char(',')?;
            }
            if f.alternate() {
                f.write_char('\n')?;
            } else if sites.peek().is_some() {
                f.write_char(' ')?;
            }
        }
        f.write_str(")")
    }
}

/// A Kappa agent declaration, e.g. `%agent: A()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentDecl {
    agent: Agent,
}

impl AgentDecl {
    pub fn new<A>(agent: A) -> Self
    where
        A: Into<Agent>,
    {
        Self {
            agent: agent.into(),
        }
    }

    pub fn agent(&self) -> &Agent {
        &self.agent
    }

    pub fn agent_mut(&mut self) -> &mut Agent {
        &mut self.agent
    }
}

impl Display for AgentDecl {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("%agent: ")
            .and(self.agent.fmt(f))
            .and(f.write_char('\n'))
    }
}

impl From<Agent> for AgentDecl {
    fn from(agent: Agent) -> Self {
        Self::new(agent)
    }
}
