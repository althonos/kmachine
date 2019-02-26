use std::iter::IntoIterator;

use super::asm::Label;
use super::asm::Register;
use super::kappa::Agent;
use super::kappa::Link;
use super::kappa::Rule;
use super::kappa::Site;

pub mod agents {

    use super::*;

    pub fn unit<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item = &'a &'a Register>,
    {
        let mut agent = agent!(UNIT());

        let mut site_prev = site!(prev[next.UNIT]);
        let mut site_r = site!(r { none });
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
        I: IntoIterator<Item = &'a &'a Register>,
    {
        // Agent with baseline sites
        let mut agent = agent!(MACHINE(ip[cm.PROG], state { run, mov, jmp }));
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
        I: IntoIterator<Item = &'a &'a Register>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(INC(prog[ins.PROG]));

        // Add one state to the r site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        agent
    }

    pub fn dec<'a, I>(registers: I) -> Agent
    where
        I: IntoIterator<Item = &'a &'a Register>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(DEC(prog[ins.PROG]));

        // Add one state to the r site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.name.to_string());
        }
        agent.site(site);

        agent
    }

    pub fn jz<'a, I, L>(registers: I, labels: L) -> Agent
    where
        I: IntoIterator<Item = &'a &'a Register>,
        L: IntoIterator<Item = &'a &'a Label>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(JZ(prog[ins.PROG]));

        // Add one state to the r site for each register
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
            agent!(MACHINE(ip[0], state { mov })),
            agent!(MACHINE(ip[0], state { run })),
        );
        rule.slot(agent!(PROG(cm[0], next[1])), agent!(PROG(cm[.], next[1])));
        rule.slot(agent!(PROG(cm[.], prev[1])), agent!(PROG(cm[0], prev[1])));

        rule
    }

    pub fn inc_zero(register: &Register) -> Rule {
        let reg = register.name.as_str();
        let mut rule = Rule::with_name(
            format!("inc({0}) | {0} == 0", register.name.as_str()).as_str(),
            1.0,
        );

        rule.slot(
            agent!(MACHINE(ip[0], state{run}, ?reg[.])),
            agent!(MACHINE(ip[0], state{mov}, ?reg[1])),
        );
        rule.slot(agent!(INC(prog[3], r{?reg})), agent!(INC(prog[3], r{?reg})));
        rule.slot(agent!(PROG(cm[0], ins[3])), agent!(PROG(cm[0], ins[3])));
        rule.slot(
            agent!(UNIT(prev[.], next[.], r{none})),
            agent!(UNIT(prev[1], next[.], r{?reg})),
        );

        rule
    }

    pub fn inc_nonzero(register: &Register) -> Rule {
        let reg = register.name.as_str();
        let mut rule = Rule::with_name(
            format!("inc({0}) | {0} != 0", register.name.as_str()).as_str(),
            1.0,
        );

        rule.slot(
            agent!(MACHINE(ip[0], state{run}, ?reg[1])),
            agent!(MACHINE(ip[0], state{mov}, ?reg[1])),
        );
        rule.slot(agent!(INC(prog[3], r{?reg})), agent!(INC(prog[3], r{?reg})));
        rule.slot(agent!(PROG(cm[0], ins[3])), agent!(PROG(cm[0], ins[3])));
        rule.slot(agent!(UNIT(prev[1])), agent!(UNIT(prev[2])));
        rule.slot(
            agent!(UNIT(prev[.], next[.], r{none})),
            agent!(UNIT(prev[1], next[2], r{?reg})),
        );

        rule
    }

    pub fn dec_zero(register: &Register) -> Rule {
        let reg = register.name.as_str();
        let mut rule = Rule::with_name(
            format!("dec({0}) | {0} == 0", register.name.as_str()).as_str(),
            1.0,
        );

        rule.slot(
            agent!(MACHINE(ip[0], state{run}, ?reg[.])),
            agent!(MACHINE(ip[0], state{mov}, ?reg[.])),
        );
        rule.slot(agent!(DEC(prog[2], r{?reg})), agent!(DEC(prog[2], r{?reg})));
        rule.slot(agent!(PROG(cm[0], ins[2])), agent!(PROG(cm[0], ins[2])));

        rule
    }

    pub fn dec_one(register: &Register) -> Rule {
        let reg = register.name.as_str();
        let mut rule = Rule::with_name(
            format!("dec({0}) | {0} == 1", register.name.as_str()).as_str(),
            1.0,
        );

        rule.slot(
            agent!(MACHINE(ip[0], state{run}, ?reg[1])),
            agent!(MACHINE(ip[0], state{mov}, ?reg[.])),
        );
        rule.slot(agent!(DEC(prog[3], r{?reg})), agent!(DEC(prog[3], r{?reg})));
        rule.slot(agent!(PROG(cm[0], ins[3])), agent!(PROG(cm[0], ins[3])));
        rule.slot(
            agent!(UNIT(prev[1], next[.], r{?reg})),
            agent!(UNIT(prev[.], next[.], r{none})),
        );

        rule
    }

    pub fn dec_more(register: &Register) -> Rule {
        let reg = register.name.as_str();
        let mut rule = Rule::with_name(
            format!("dec({0}) | {0} > 1", register.name.as_str()).as_str(),
            1.0,
        );

        rule.slot(
            agent!(MACHINE(ip[0], state{run}, ?reg[1])),
            agent!(MACHINE(ip[0], state{mov}, ?reg[2])),
        );
        rule.slot(agent!(DEC(prog[3], r{?reg})), agent!(DEC(prog[3], r{?reg})));
        rule.slot(agent!(PROG(cm[0], ins[3])), agent!(PROG(cm[0], ins[3])));
        rule.slot(
            agent!(UNIT(prev[1], next[2], r{?reg})),
            agent!(UNIT(prev[.], next[.], r{none})),
        );
        rule.slot(agent!(UNIT(prev[2])), agent!(UNIT(prev[2])));

        rule
    }

}
