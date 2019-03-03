use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use indexmap::IndexSet;

use super::Link;
use super::State;

/// A Kappa site, e.g. `s{a}[.]`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Site {
    name: String,
    states: IndexSet<State>,
    links: IndexSet<Link>,
}

impl Site {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            states: IndexSet::new(),
            links: IndexSet::new(),
        }
    }

    pub fn link(&mut self, link: Link) -> &mut Self {
        self.links.insert(link);
        self
    }

    pub fn linkable<A, S>(&mut self, agent: A, site: S) -> &mut Self
    where
        A: Into<String>,
        S: Into<String>,
    {
        self.links.insert(Link::BoundTo {
            agent: agent.into(),
            site: site.into(),
        });
        self
    }

    pub fn state<S>(&mut self, state: S) -> &mut Self
    where
        S: Into<State>,
    {
        self.states.insert(state.into());
        self
    }
}

impl Display for Site {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(&self.name)?;
        if !self.states.is_empty() {
            let states = &mut self.states.iter().peekable();
            f.write_char('{')?;
            while let Some(ref state) = states.next() {
                if states.peek().is_some() {
                    write!(f, "{}, ", state)?;
                } else {
                    write!(f, "{}", state)?;
                }
            }
            f.write_char('}')?;
        }
        if !self.links.is_empty() {
            let links = &mut self.links.iter().peekable();
            f.write_char('[')?;
            while let Some(ref link) = links.next() {
                if links.peek().is_some() {
                    write!(f, "{}, ", link)?;
                } else {
                    write!(f, "{}", link)?;
                }
            }
            f.write_char(']')?;
        }
        Ok(())
    }
}
