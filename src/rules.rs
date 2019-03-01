use std::iter::IntoIterator;

use asm::Label;
use asm::Register;
use kappa::Agent;
use kappa::Link;
use kappa::Rule;
use kappa::Site;

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

pub fn jmp(label: &str) -> Rule {
    let name = format!("jmp({0})", label);

    rule!(
        ?name {
            MACHINE(ip[0], state{run}, target{_none}),
            PROG(cm[0], ins[1]),
            JMP(prog[1], l{?label}),
        } => {
            MACHINE(ip[.], state{jmp}, target{?label}),
            PROG(cm[.], ins[1]),
            JMP(prog[1], l{?label}),
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