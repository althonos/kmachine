use std::iter::IntoIterator;

use kmachine_asm::Label;
use kmachine_asm::Register;

use kmachine_kappa::Agent;
use kmachine_kappa::Link;
use kmachine_kappa::Rule;
use kmachine_kappa::Site;

pub mod agents {

    use super::*;

    pub fn unit<R, SR>(registers: R) -> Agent
    where
        R: IntoIterator<Item = SR>,
        SR: AsRef<str>,
    {
        let mut agent = agent!(UNIT(next[prev.UNIT]));

        let mut site_prev = site!(prev[next.UNIT]);
        let mut site_r = site!(r { _none });
        for register in registers.into_iter() {
            site_r.state(register.as_ref());
            site_prev.linkable("MACHINE", register.as_ref());
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
                ins[prog.INC, prog.DEC, prog.JZ, prog.LBL, prog.CLR]
            )
        )
    }

    pub fn machine<R, SR, L, SL>(registers: R, labels: L) -> Agent
    where
        R: IntoIterator<Item = SR>,
        L: IntoIterator<Item = SL>,
        SR: AsRef<str>,
        SL: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(MACHINE(ip[cm.PROG], state { run, mov, jmp }));
        // Add one site for each register
        for register in registers.into_iter() {
            site = Site::new(register.as_ref());
            site.linkable("UNIT", "prev");
            agent.site(site);
        }
        // Add one state for each
        site = site!(target { _none });
        for label in labels.into_iter() {
            site.state(label.as_ref());
        }
        agent.site(site);

        agent
    }

    pub fn lbl<L, SL>(labels: L) -> Agent
    where
        L: IntoIterator<Item = SL>,
        SL: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(LBL(prog[ins.PROG]));

        // Add one state to the l site for each label
        site = site!(l);
        for label in labels.into_iter() {
            site.state(label.as_ref());
        }
        agent.site(site);

        agent
    }

    pub fn inc<R, SR>(registers: R) -> Agent
    where
        R: IntoIterator<Item = SR>,
        SR: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(INC(prog[ins.PROG]));

        // Add one state to the r site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.as_ref());
        }
        agent.site(site);

        agent
    }

    pub fn dec<R, SR>(registers: R) -> Agent
    where
        R: IntoIterator<Item = SR>,
        SR: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(DEC(prog[ins.PROG]));

        // Add one state to the r site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.as_ref());
        }
        agent.site(site);

        agent
    }

    pub fn jz<R, SR, L, SL>(registers: R, labels: L) -> Agent
    where
        R: IntoIterator<Item = SR>,
        L: IntoIterator<Item = SL>,
        SR: AsRef<str>,
        SL: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(JZ(prog[ins.PROG]));

        // Add one state to the `r` site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.as_ref());
        }
        agent.site(site);

        // Add one state to the `l` site for each label
        site = site!(l);
        for label in labels.into_iter() {
            site.state(label.as_ref());
        }
        agent.site(site);

        agent
    }

    pub fn clr<R, SR>(registers: R) -> Agent
    where
        R: IntoIterator<Item = SR>,
        SR: AsRef<str>,
    {
        // Agent with baseline sites
        let mut site;
        let mut agent = agent!(CLR(prog[ins.PROG]));

        // Add one state to the `r` site for each register
        site = site!(r);
        for register in registers.into_iter() {
            site.state(register.as_ref());
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
        let name = format!("bind | target == {0}", label);

        rule!(
            ?name {
                MACHINE(ip[.], state{jmp}, target{?label}),
                PROG(cm[.], ins[0]),
                LBL(prog[0], l{?label})
            } => {
                MACHINE(ip[1], state{run}, target{_none}),
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
                UNIT(prev[2]),
                UNIT(prev[.], next[.], r{_none}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[2]),
                PROG(cm[0], ins[1]),
                INC(prog[1], r{?reg}),
                UNIT(prev[3]),
                UNIT(prev[2], next[3], r{?reg}),
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

    pub fn clr_zero(reg: &str) -> Rule {
        let name = format!("clr({0}) | {0} == 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[.]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg})
            } => {
                MACHINE(ip[0], state{mov}, ?reg[.]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg})
            } @ 1.0
        )
    }

    pub fn clr_one(reg: &str) -> Rule {
        let name = format!("clr({0}) | {0} == 1", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
                UNIT(prev[2], next[.], r{?reg}),
            } => {
                MACHINE(ip[0], state{mov}, ?reg[.]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
                UNIT(prev[.], next[.], r{_none}),
            } @ 1.0
        )
    }

    pub fn clr_more(reg: &str) -> Rule {
        let name = format!("clr({0}) | {0} >= 2", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
                UNIT(prev[2], next[3], r{?reg}),
                UNIT(prev[3]),
            } => {
                MACHINE(ip[0], state{run}, ?reg[3]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
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
