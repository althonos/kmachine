#![allow(unused_macros, unused_imports)]

extern crate indexmap;
#[macro_use]
extern crate kmachine_kappa;
extern crate kmachine_asm;

mod compile;

use std::borrow::Cow;
use std::io::Read;

use indexmap::IndexSet;
use kmachine_asm::AttParser;
use kmachine_asm::Arg;
use kmachine_asm::AsmParser;
use kmachine_asm::Label;
use kmachine_asm::Line;
use kmachine_asm::Register;
use kmachine_kappa::Agent;
use kmachine_kappa::Init;
use kmachine_kappa::KappaProgram;
use kmachine_kappa::Site;
use kmachine_kappa::Observable;
use kmachine_kappa::Pattern;
use kmachine_kappa::AlgebraicExpression;

fn main() {
    for filename in std::env::args().skip(1) {
        let mut file = std::fs::File::open(&filename).unwrap();
        let mut program = String::new();
        file.read_to_string(&mut program).unwrap();

        // Parse the ASM program
        let asm = AttParser::parse_asm(&program);

        // Collect all registers used in the program.
        let registers: IndexSet<&Cow<_>> = asm.registers().into_iter().map(|r| &r.name).collect();

        // Collect all labels declared in the program.
        let labels: IndexSet<&Cow<_>> = asm.labels().into_iter().map(|l| &l.name).collect();

        // Compile the CM program into a Kappa source
        let program = {
            let mut program = KappaProgram::new();

            // Build agents
            program
                // counter units
                .agent(compile::agents::unit(&registers))
                // machine
                .agent(compile::agents::machine(&registers, &labels))
                // instructions
                .agent(compile::agents::prog())
                // pseudo-operands
                .agent(compile::agents::lbl(&labels))
                .agent(compile::agents::clr(&registers))
                .agent(compile::agents::inc(&registers))
                .agent(compile::agents::dec(&registers))
                .agent(compile::agents::jz(&registers, &labels));

            // Build static rules
            program
                .rule(compile::rules::mov())
                .rule(compile::rules::lbl());
            // Build register-dependent rules
            for register in registers.iter() {
                program
                    .rule(compile::rules::inc_nonzero(register))
                    .rule(compile::rules::inc_zero(register))
                    .rule(compile::rules::dec_zero(register))
                    .rule(compile::rules::dec_one(register))
                    .rule(compile::rules::dec_more(register))
                    .rule(compile::rules::jz_nonzero(register))
                    .rule(compile::rules::clr_zero(register))
                    .rule(compile::rules::clr_one(register))
                    .rule(compile::rules::clr_more(register));
            }
            // Build label-dependent rules
            for label in labels.iter() {
                program.rule(compile::rules::bind(label));
            }
            // Build label-register-dependent rules
            for label in labels.iter() {
                for register in registers.iter() {
                    program.rule(compile::rules::jz_zero(register, label));
                }
            }

            // Build static init
            program.init(Init::with_agent(
                1,
                agent!(UNIT(prev[.], next[.], r{_none})),
            ));
            // Build program polymer
            let mut program_chain = Init::new(1);
            program_chain.agent(agent!(MACHINE(state { run }, ip[0])));
            let mut lines = asm.lines().iter().enumerate().peekable();
            while let Some((index, line)) = lines.next() {
                //
                let idx_prev = index * 2;
                let idx_prog = idx_prev + 1;
                let idx_next = idx_prev + 2;

                program_chain.agent(if idx_prev == 0 {
                    agent!(PROG( cm[0], ins[?idx_prog], next[?idx_next]))
                } else if lines.peek().is_none() {
                    agent!(PROG( prev[?idx_prev], ins[?idx_prog]))
                } else {
                    agent!(PROG( prev[?idx_prev], ins[?idx_prog], next[?idx_next]))
                });

                program_chain.agent(match line {
                    Line::LabelLine(l) => {
                        let label = l.name.as_ref();
                        agent!(LBL(prog[?idx_prog], l{?label}))
                    }
                    Line::OpLine(ins) => match ins.op() {
                        opname @ "clr" | opname @ "inc" | opname @ "dec" => {
                            let name = opname.to_uppercase();
                            let register = match ins.arguments().first() {
                                Some(Arg::Register(r)) => r.name.as_ref(),
                                Some(arg) => panic!(
                                    "invalid argument #1 for instruction `{}`: {:?}",
                                    opname, arg
                                ),
                                None => panic!("missing argument for instruction `{}`"),
                            };
                            agent!(?name (prog[?idx_prog], r{?register}))
                        }
                        // "dec" => {
                        //     let register = op.arguments().first().unwrap().name.as_ref();
                        //     agent!(DEC(prog[?idx_prog], r{?register}))
                        // }
                        // "inc" => {
                        //     let register = op.arguments().first().unwrap().name.as_ref();
                        //     agent!(INC(prog[?idx_prog], r{?register}))
                        // }
                        opname @ "jz" => {
                            let mut args = ins.arguments().iter();
                            let register = match args.next() {
                                Some(Arg::Register(r)) => r.name.as_ref(),
                                Some(arg) => panic!(
                                    "invalid argument #1 for instruction `{}`: {:?}",
                                    opname, arg
                                ),
                                None => panic!("missing argument for instruction `{}`", opname),
                            };
                            let label = match args.next() {
                                Some(Arg::Label(l)) => l.name.as_ref(),
                                Some(arg) => panic!(
                                    "invalid argument #2 for instruction `{}`: {:?}",
                                    opname, arg
                                ),
                                None => panic!("missing argument for instruction `{}`", opname),
                            };
                            assert!(labels.contains(&Cow::from(label)));
                            agent!(JZ(prog[?idx_prog], r{?register}, l{?label}))
                        }

                        opname => panic!("unknown instruction `{}`", opname),
                    },
                });
            }
            program.init(program_chain);


            // Build observables
            for register in registers.iter() {
                let r: &str = register.as_ref();
                let pattern = Pattern::from(vec![agent!(UNIT(r{?r}))]);
                let obs = Observable::new(
                    r,
                    AlgebraicExpression::Occurrences(pattern)
                );
                program.observable(obs);
            }

            program
        };

        println!("// {}\n{:#}", filename, program);
    }
}
