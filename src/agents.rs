use std::iter::IntoIterator;

use asm::Label;
use asm::Register;
use kappa::Agent;
use kappa::Link;
use kappa::Rule;
use kappa::Site;

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

pub fn jmp<L, SL>(labels: L) -> Agent
where
    L: IntoIterator<Item = SL>,
    SL: AsRef<str>,
{
    // Agent with baseline sites
    let mut site;
    let mut agent = agent!(JMP(prog[ins.PROG]));

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
