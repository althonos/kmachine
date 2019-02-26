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
        let mut agent = agent!(UNIT(next[prev.UNIT]));

        let mut site_prev = site!(prev[next.UNIT]);
        let mut site_r = site!(r { _none });
        for register in registers.into_iter() {
            site_r.state(register.name.as_str());
            site_prev.linkable("MACHINE", register.name.as_str());
        }

        agent.site(site_prev);
        agent.site(site_r);
        agent
    }

    pub fn prog() -> Agent {
        agent!(
            PROG(
                prev[next.PROG],
                next[prev.PROG],
                cm[ip.MACHINE],
                ins[prog.INC, prog.DEC, prog.JZ, prog.LBL]
            )
        )
    }

    pub fn machine<'a, I, L>(registers: I, labels: L) -> Agent
    where
        I: IntoIterator<Item = &'a &'a Register>,
        L: IntoIterator<Item = &'a &'a Label>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(MACHINE(ip[cm.PROG], state { run, mov, jmp }));
        // Add one site for each register
        for register in registers.into_iter() {
            site = Site::new(register.name.as_str());
            site.linkable("UNIT", "prev");
            agent.site(site);
        }
        // Add one state for each
        site = site!(target { _none });
        for label in labels.into_iter() {
            site.state(label.name.as_str());
        }
        agent.site(site);

        agent
    }

    pub fn lbl<'a, L>(labels: L) -> Agent
    where
        L: IntoIterator<Item = &'a &'a Label>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(LBL(prog[ins.PROG]));

        // Add one state to the l site for each label
        site = site!(l);
        for label in labels.into_iter() {
            site.state(label.name.to_string());
        }
        agent.site(site);

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
        rule!(
            "mov" {
                MACHINE(ip[0], state{mov}),
                PROG(cm[0], next[1]),
                PROG(cm[.], prev[1]),
            } => {
                MACHINE(ip[0], state{run}),
                PROG(cm[.], next[1]),
                PROG(cm[0], prev[1]),
            } @ 1.0
        )
    }

    pub fn bind(label: &str) -> Rule {
        rule!(
            "bind" {
                MACHINE(ip[.], state{jmp}, target{?label}),
                PROG(cm[.], ins[0]),
                LBL(prog[0], l{?label})
            } => {
                MACHINE(ip[1], state{run}, target{?label}),
                PROG(cm[1], ins[0]),
                LBL(prog[0], l{?label})
            } @ 1.0
        )
    }

    pub fn lbl() -> Rule {
        rule!(
            "label" {
                MACHINE(ip[0], state{run}),
                PROG(cm[0], ins[1]),
                LBL(prog[1])
            } => {
                MACHINE(ip[0], state{mov}),
                PROG(cm[0], ins[1]),
                LBL(prog[1])
            } @ 1.0
        )
    }

    pub fn inc_zero(reg: &str) -> Rule {
        let name = format!("inc({0}) | {0} == 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[.]),
                PROG(cm[0], ins[1]),
                INC(prog[1], r{?reg}),
                UNIT(prev[.], next[.], r{_none}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[2]),
                PROG(cm[0], ins[1]),
                INC(prog[1], r{?reg}),
                UNIT(prev[2], next[.], r{?reg}),
            } @ 1.0
        )
    }

    pub fn inc_nonzero(reg: &str) -> Rule {
        let name = format!("inc({0}) | {0} != 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                INC(prog[1], r{?reg}),
                UNIT(prev[2], next[.]),
                UNIT(prev[.], next[.], r{_none}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[2]),
                PROG(cm[0], ins[1]),
                INC(prog[1], r{?reg}),
                UNIT(prev[2], next[3]),
                UNIT(prev[3], next[.], r{?reg}),
            } @ 1.0
        )
    }

    pub fn dec_zero(reg: &str) -> Rule {
        let name = format!("dec({0}) | {0} == 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[.]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg})
            } => {
                MACHINE(ip[0], state{mov}, ?reg[.]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg})
            } @ 1.0
        )
    }

    pub fn dec_one(reg: &str) -> Rule {
        let name = format!("dec({0}) | {0} == 1", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg}),
                UNIT(prev[2], next[.], r{?reg}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[.]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg}),
                UNIT(prev[.], next[.], r{_none}),
            } @ 1.0
        )
    }

    pub fn dec_more(reg: &str) -> Rule {
        let name = format!("dec({0}) | {0} >= 2", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg}),
                UNIT(prev[2], next[3], r{?reg}),
                UNIT(prev[3]),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[3]),
                PROG(cm[0], ins[1]),
                DEC(prog[1], r{?reg}),
                UNIT(prev[.], next[.], r{_none}),
                UNIT(prev[3]),
            } @ 1.0
        )
    }

    pub fn jz_nonzero(reg: &str) -> Rule {
        let name = format!("jz({0}, *) | {0} != 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[_]),
                PROG(cm[0], ins[1]),
                JZ(prog[1], r{?reg}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[_]),
                PROG(cm[0], ins[1]),
                JZ(prog[1], r{?reg}),
            } @ 1.0
        )
    }

    pub fn jz_zero(reg: &str, label: &str) -> Rule {
        let name = format!("jz({0}, {1}) | {0} == 0", reg, label);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[.], target{_none}),
                PROG(cm[0], ins[1]),
                JZ(prog[1], r{?reg}, l{?label}),
            } => {
                MACHINE(ip[.], state{jmp}, ?reg[.], target{?label}),
                PROG(cm[.], ins[1]),
                JZ(prog[1], r{?reg}, l{?label}),
            } @ 1.0
        )
    }

}
