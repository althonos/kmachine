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
        let mut agent = agent!(UNIT());

        let mut site_prev = site!(prev[next.UNIT]);
        let mut site_r = site!(r{none});
        for register in registers.into_iter() {
            site_r.state(register.name.as_str());
            site_prev.linkable("MACHINE", register.name.as_str());
        }

        agent.site(site_prev);
        agent.site(site_r);
        agent.site(site!(next[prev.UNIT]));
        agent
    }

    pub fn prog() -> Agent {
        agent!(
            PROG(
                prev[next.PROG],
                next[prev.PROG],
                cm[ip.MACHINE],
                ins[prog.INC, prog.DEC, prog.JZ]
            )
        )
    }

    pub fn machine<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item=&'a &'a Register>,
    {
        // Agent with baseline sites
        let mut agent = agent!(MACHINE(ip[cm.PROG], state{run, mov, jmp}));
        // Add one site for each register
        let mut site;
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
        let mut agent = agent!(INC(prog[ins.PROG]));

        site = site!(r);
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
        let mut agent = agent!(DEC(prog[ins.PROG]));

        site = site!(r);
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
        let mut agent = agent!(JZ(prog[ins.PROG]));

        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        site = site!(l);
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
        let mut rule = Rule::with_name("move", 1.0);
        rule.slot(
            agent!(MACHINE(ip[0], state{mov})),
            agent!(MACHINE(ip[0], state{run}))
        );
        rule.slot(
            agent!(PROG(cm[0], next[1])),
            agent!(PROG(cm[.], next[1]))
        );
        rule.slot(
            agent!(PROG(cm[.], prev[1])),
            agent!(PROG(cm[0], prev[1]))
        );
        rule
    }

    pub fn inc_nonzero(register: &Register) -> Rule {
        let mut rule = Rule::with_name(
            format!("inc({0}) | {0} != 0", register.name.as_str()).as_str(),
            1.0
        );

        let mut machine_left = agent!(MACHINE(ip[0], state{run}));
        let mut machine_right = agent!(MACHINE(ip[0], state{mov}));
        let mut site_register = Site::new(register.name.as_str());
        site_register.link(link!(1));
        machine_left.site(site_register.clone());
        machine_right.site(site_register);
        rule.slot(machine_left, machine_right);

        let mut inc = agent!(INC(prog[3]));
        let mut state_register = site!(r);
        state_register.state(register.name.as_str());
        inc.site(state_register.clone());
        rule.slot(inc.clone(), inc);

        rule.slot(
            agent!(PROG(cm[0], ins[3])),
            agent!(PROG(cm[0], ins[3])),
        );

        rule.slot(
            agent!(UNIT(prev[1])),
            agent!(UNIT(prev[2]))
        );

        let mut new_unit = agent!(UNIT(prev[1], next[2]));
        new_unit.site(state_register);
        rule.slot(agent!(UNIT(prev[.], next[.], r{none})), new_unit);

        rule
    }

    pub fn inc_zero(register: &Register) -> Rule {
        let mut rule = Rule::with_name(
            format!("inc({0}) | {0} == 0", register.name.as_str()).as_str(),
            1.0
        );

        let mut machine_left = agent!(MACHINE(ip[0], state{run}));
        let mut machine_right = agent!(MACHINE(ip[0], state{mov}));

        let mut register_left = Site::new(register.name.as_str());
        let mut register_right = register_left.clone();
        register_left.link(link!(.));
        register_right.link(link!(1));
        machine_left.site(register_left);
        machine_right.site(register_right);
        rule.slot(machine_left, machine_right);

        let mut inc = agent!(INC(prog[3]));
        let mut state_register = site!(r);
        state_register.state(register.name.as_str());
        inc.site(state_register.clone());
        rule.slot(inc.clone(), inc);

        rule.slot(
            agent!(PROG(cm[0], ins[3])),
            agent!(PROG(cm[0], ins[3])),
        );

        let mut new_unit = agent!(UNIT(prev[1], next[.]));
        new_unit.site(state_register);
        rule.slot(agent!(UNIT(prev[.], next[.], r{none})), new_unit);

        rule
    }

}
