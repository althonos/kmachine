use std::iter::IntoIterator;

use super::asm::Register;
use super::asm::Label;
use super::kappa::Agent;
use super::kappa::Link;
use super::kappa::Rule;
use super::kappa::Site;

pub mod agents {

    use super::*;

    pub fn unit<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
    {
        let mut agent = Agent::new("UNIT");

        let mut site_prev = Site::new("prev");
        site_prev.linkable("UNIT", "next");
        let mut site_r = Site::new("r");
        for register in registers.into_iter() {
            site_prev.linkable("MACHINE", register.name.as_str());
            site_r.state(register.name.as_str());
        }
        agent.site(site_prev);
        agent.site(site_r);

        let mut site_next = Site::new("next");
        site_next.linkable("UNIT", "prev");
        agent.site(site_next);

        agent
    }

    pub fn prog() -> Agent {
        let mut site;
        let mut agent = Agent::new("PROG");

        site = Site::new("prev");
        site.linkable("PROG", "next");
        agent.site(site);

        site = Site::new("next");
        site.linkable("PROG", "prev");
        agent.site(site);

        site = Site::new("cm");
        site.linkable("MACHINE", "ip");
        agent.site(site);

        // NB: add more commands here
        site = Site::new("ins");
        site.linkable("INS", "prog");
        site.linkable("DEC", "prog");
        site.linkable("JZ", "prog");
        agent.site(site);

        agent
    }

    pub fn machine<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
    {
        let mut site;
        let mut agent = Agent::new("MACHINE");

        site = Site::new("ip");
        site.linkable("PROG", "cm");
        agent.site(site);

        // NB: add more commands here
        site = Site::new("state");
        site.state("run");
        site.state("mov");
        site.state("jmp");
        agent.site(site);

        for register in registers.into_iter() {
            site = Site::new(register.name.as_str());
            site.linkable("UNIT", "prev");
            agent.site(site);
        }

        agent
    }

    pub fn inc<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
    {
        let mut site;
        let mut agent = Agent::new("INC");

        site = Site::new("prog");
        site.linkable("PROG", "ins");
        agent.site(site);

        site = Site::new("r");
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        agent
    }

    pub fn dec<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
    {
        let mut site;
        let mut agent = Agent::new("DEC");

        site = Site::new("prog");
        site.linkable("PROG", "ins");
        agent.site(site);

        site = Site::new("r");
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        agent
    }

    pub fn jz<'a, I, L>(registers: I, labels: L) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
        L: IntoIterator<Item=&'a &'a Label>,
    {
        let mut site;
        let mut agent = Agent::new("JZ");

        site = Site::new("prog");
        site.linkable("PROG", "ins");
        agent.site(site);

        site = Site::new("r");
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        site = Site::new("l");
        for label in labels.into_iter() {
            site.state(label.name.to_string());
        }
        agent.site(site);

        agent
    }

}


pub mod rules {

    use super::*;

    pub fn mov() -> Rule {

        let mut site;
        let mut left;
        let mut right;

        let mut rule = Rule::with_name("move", 1.0);

        left = Agent::new("MACHINE");
        site = Site::new("ip");
        site.link(Link::Numbered(0));
        left.site(site);
        site = Site::new("state");
        site.state("move");
        left.site(site);
        right = Agent::new("MACHINE");
        site = Site::new("ip");
        site.link(Link::Numbered(2));
        right.site(site);
        site = Site::new("state");
        site.state("run");
        right.site(site);
        rule.agent(left, right);

        left = Agent::new("PROG");
        site = Site::new("cm");
        site.link(Link::Numbered(0));
        left.site(site);
        site = Site::new("next");
        site.link(Link::Numbered(1));
        left.site(site);
        right = Agent::new("PROG");
        site = Site::new("cm");
        site.link(Link::Free);
        right.site(site);
        site = Site::new("next");
        site.link(Link::Numbered(1));
        right.site(site);
        rule.agent(left, right);

        rule

    }
}
