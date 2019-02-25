
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Site {
    name: String,
    states: IndexSet<String>,
    bindings: IndexSet<Stub>,
}

impl Site {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            states: IndexSet::new(),
            bindings: IndexSet::new(),
        }
    }

    pub fn binding<A, S>(&mut self, agent: A, site: S) -> &mut Self
    where
        A: Into<String>,
        S: Into<String>,
    {
        self.bindings.insert(Stub::new(agent, site));
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Stub {
    agent: String,
    site: String,
}

impl Stub {
    pub fn new<A, S>(agent: A, site: S) -> Self
    where
        A: Into<String>,
        S: Into<String>,
    {
        Self {
            agent: agent.into(),
            site: site.into(),
        }
    }
}
