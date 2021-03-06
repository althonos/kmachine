use attasm::ast::Arg;
use attasm::ast::Line;
use attasm::ast::Program;
use kappa::Init;

pub fn units(count: usize) -> Init {
    Init::with_agent(count, agent!(UNIT(prev[.], next[.], r{_none})))
}

pub fn program(asm: &Program) -> Init {
    let mut chain = Init::new(1);
    chain.agent(agent!(MACHINE(state { run }, ip[0])));

    let mut lines = asm.lines().iter().enumerate().peekable();
    while let Some((index, line)) = lines.next() {
        let idx_prev = index * 2;
        let idx_prog = idx_prev + 1;
        let idx_next = idx_prev + 2;

        chain.agent(match (index, lines.peek()) {
            (0, _) => agent!(PROG( cm[0], ins[?idx_prog], next[?idx_next])),
            (_, None) => agent!(PROG( prev[?idx_prev], ins[?idx_prog])),
            (_, _) => agent!(PROG( prev[?idx_prev], ins[?idx_prog], next[?idx_next])),
        });

        chain.agent(match line {
            Line::LabelLine(l) => {
                let label = l.name().as_ref();
                agent!(LBL(prog[?idx_prog], l{?label}))
            }
            Line::OpLine(ins) => match ins.mnemonic() {
                "add" => {
                    let (src, dst) = args!(ins, add(Arg::Register, Arg::Register));
                    let s = src.name().as_ref();
                    let d = dst.name().as_ref();
                    if src == dst {
                        panic!("invalid arguments for instruction `add`: {}, {}", s, d);
                    }
                    agent!(ADD(prog[?idx_prog], src{?s}, dst{?d}))
                }
                "clr" => {
                    let (register,) = args!(ins, clr(Arg::Register));
                    let r = register.name().as_ref();
                    agent!(CLR(prog[?idx_prog], r{?r}))
                }
                "dec" => {
                    let (register,) = args!(ins, dec(Arg::Register));
                    let r = register.name().as_ref();
                    agent!(DEC(prog[?idx_prog], r{?r}))
                }
                "inc" => {
                    let (register,) = args!(ins, inc(Arg::Register));
                    let r = register.name().as_ref();
                    agent!(INC(prog[?idx_prog], r{?r}))
                }
                "jmp" => {
                    let (label,) = args!(ins, jmp(Arg::Label));
                    let l = label.name().as_ref();
                    agent!(JMP(prog[?idx_prog], l{?l}))
                }

                "jnz" => {
                    let (register, label) = args!(ins, jz(Arg::Register, Arg::Label));
                    let r = register.name().as_ref();
                    let l = label.name().as_ref();
                    agent!(JNZ(prog[?idx_prog], r{?r}, l{?l}))
                }
                "jz" => {
                    let (register, label) = args!(ins, jz(Arg::Register, Arg::Label));
                    let r = register.name().as_ref();
                    let l = label.name().as_ref();
                    agent!(JZ(prog[?idx_prog], r{?r}, l{?l}))
                }
                "mov" => {
                    let (src, dst) = args!(ins, mov(Arg::Register, Arg::Register));
                    let s = src.name().as_ref();
                    let d = dst.name().as_ref();
                    if src == dst {
                        panic!("invalid arguments for instruction `mov`: {}, {}", s, d);
                    }
                    agent!(MOV(prog[?idx_prog], src{?s}, dst{?d}))
                }
                "swp" => {
                    let (src, dst) = args!(ins, mov(Arg::Register, Arg::Register));
                    let s = src.name().as_ref();
                    let d = dst.name().as_ref();
                    if src == dst {
                        panic!("invalid arguments for instruction `swp`: {}, {}", s, d);
                    }
                    agent!(SWP(prog[?idx_prog], src{?s}, dst{?d}))
                }
                opname => panic!("unknown instruction `{}`", opname),
            },
        });
    }

    chain
}
