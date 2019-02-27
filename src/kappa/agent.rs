use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use indexmap::IndexSet;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Site {
    name: String,
    states: IndexSet<String>,
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
        S: Into<String>,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Link {
    Unknown,
    Free,
    Numbered(usize),
    Bound,
    BoundTo { agent: String, site: String },
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Link::Unknown => f.write_char('#'),
            Link::Free => f.write_char('.'),
            Link::Numbered(n) => n.fmt(f),
            Link::Bound => f.write_char('_'),
            Link::BoundTo { site, agent } => write!(f, "{}.{}", site, agent),
        }
    }
}
