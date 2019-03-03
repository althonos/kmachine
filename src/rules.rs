use kappa::Rule;

pub fn next() -> Rule {
    rule!(
        "move" {
            MACHINE(ip[0], state{next}),
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
            MACHINE(ip[.], state{bind}, target{?label}),
            PROG(cm[.], ins[0]),
            LBL(prog[0], l{?label})
        } => {
            MACHINE(ip[1], state{run}, target{_none}),
            PROG(cm[1], ins[0]),
            LBL(prog[0], l{?label})
        } @ 1.0
    )
}

pub fn reset_units() -> Rule {
    rule!(
        "reset_units" {
            UNIT(prev[.], next[0], r{#}),
            UNIT(prev[0], r{#})
        } => {
            UNIT(prev[.], next[.], r{_none}),
            UNIT(prev[.], r{_none})
        } @ 1.0
    )
}

pub fn relabel_units(r1: &str, r2: &str) -> Rule {
    let name = format!("relabel_units | {0} -> {1}", r1, r2);

    rule!(
         ?name {
            UNIT(prev[_], next[0], r{?r1}),
            UNIT(prev[0], r{?r2})
        } => {
            UNIT(prev[_], next[0], r{?r1}),
            UNIT(prev[0], r{?r1})
        } @ 1.0
    )
}

pub mod instructions {

    use super::Rule;

    pub fn lbl() -> Rule {
        rule!(
            "label" {
                MACHINE(ip[0], state{run}),
                PROG(cm[0], ins[1]),
                LBL(prog[1])
            } => {
                MACHINE(ip[0], state{next}),
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
                MACHINE(ip[0], state{next}, ?reg[2]),
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
                MACHINE(ip[0], state{next}, ?reg[2]),
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
                MACHINE(ip[0], state{next}, ?reg[.]),
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
                MACHINE(ip[0], state{next}, ?reg[.]),
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
                MACHINE(ip[0], state{next}, ?reg[3]),
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
                MACHINE(ip[0], state{next}, ?reg[.]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg})
            } @ 1.0
        )
    }

    pub fn clr_nonzero(reg: &str) -> Rule {
        let name = format!("clr({0}) | {0} == 1", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[2]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
                UNIT(prev[2], r{?reg}),
            } => {
                MACHINE(ip[0], state{next}, ?reg[.]),
                PROG(cm[0], ins[1]),
                CLR(prog[1], r{?reg}),
                UNIT(prev[.], r{_none}),
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
                MACHINE(ip[.], state{bind}, target{?label}),
                PROG(cm[.], ins[1]),
                JMP(prog[1], l{?label}),
            } @ 1.0
        )
    }

    pub fn jnz_nonzero(reg: &str, label: &str) -> Rule {
        let name = format!("jnz({0}, {1}) | {0} != 0", reg, label);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[_], target{_none}),
                PROG(cm[0], ins[1]),
                JNZ(prog[1], r{?reg}, l{?label}),
            } => {
                MACHINE(ip[.], state{bind}, ?reg[_], target{?label}),
                PROG(cm[.], ins[1]),
                JNZ(prog[1], r{?reg}, l{?label}),
            } @ 1.0
        )
    }

    pub fn jnz_zero(reg: &str) -> Rule {
        let name = format!("jnz({0}, *) | {0} != 0", reg);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?reg[.]),
                PROG(cm[0], ins[1]),
                JNZ(prog[1], r{?reg})
            } => {
                MACHINE(ip[0], state{next}, ?reg[.]),
                PROG(cm[0], ins[1]),
                JNZ(prog[1], r{?reg})
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
                MACHINE(ip[0], state{next}, ?reg[_]),
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
                MACHINE(ip[.], state{bind}, ?reg[.], target{?label}),
                PROG(cm[.], ins[1]),
                JZ(prog[1], r{?reg}, l{?label}),
            } @ 1.0
        )
    }

    pub fn mov_zero(src: &str, dst: &str) -> Rule {
        let name = format!("mov({0}, {1}) | {0} == 0", src, dst);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?src[.], ?dst[#]),
                PROG(cm[0], ins[1]),
                MOV(prog[1], src{?src}, dst{?dst}),
            } => {
                MACHINE(ip[0], state{next}, ?src[.], ?dst[.]),
                PROG(cm[0], ins[1]),
                MOV(prog[1], src{?src}, dst{?dst}),
            } @ 1.0
        )
    }

    pub fn mov_nonzero(src: &str, dst: &str) -> Rule {
        let name = format!("mov({0}, {1}) | {0} != 0", src, dst);

        rule!(
            ?name {
                MACHINE(ip[0], state{run}, ?src[2], ?dst[#]),
                PROG(cm[0], ins[1]),
                MOV(prog[1], src{?src}, dst{?dst}),
                UNIT(prev[2], r{?src}),
            } => {
                MACHINE(ip[0], state{next}, ?src[.], ?dst[2]),
                PROG(cm[0], ins[1]),
                MOV(prog[1], src{?src}, dst{?dst}),
                UNIT(prev[2], r{?dst}),
            } @ 1.0
        )
    }

}
