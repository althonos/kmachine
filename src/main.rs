#![allow(unused_macros, unused_imports)]

#[macro_use]
extern crate pest_derive;

extern crate indexmap;
extern crate pest;

#[macro_use]
pub mod kappa;

pub mod asm;
pub mod compile;

use std::io::Read;

use indexmap::IndexSet;

use self::asm::att::AttParser;
use self::asm::AsmParser;
use self::asm::Label;
use self::asm::Line;
use self::asm::Op;
use self::asm::Register;
use self::kappa::Agent;
use self::kappa::KappaProgram;
use self::kappa::Site;

fn main() {
    for filename in std::env::args().skip(1) {
        let mut file = std::fs::File::open(&filename).unwrap();
        let mut program = String::new();
        file.read_to_string(&mut program).unwrap();

        // Parse the ASM program
        let asm = AttParser::parse_asm(&program);

        // Collect all registers used in the program.
        let registers: IndexSet<&Register> = asm
            .iter()
            .flat_map(|line| match line {
                Line::LabelLine(_) => None,
                Line::OpLine(op) => match op {
                    Op::Inc(r) => Some(r),
                    Op::Dec(r) => Some(r),
                    Op::Jz(r, _) => Some(r),
                },
            })
            .collect();

        // Collect all labels declared in the program.
        let labels: IndexSet<&Label> = asm
            .iter()
            .flat_map(|line| match line {
                Line::LabelLine(l) => Some(l),
                Line::OpLine(_) => None,
            })
            .collect();

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
                .agent(compile::agents::inc(&registers))
                .agent(compile::agents::dec(&registers))
                .agent(compile::agents::jz(&registers, &labels));

            // Build static rules
            program.rule(compile::rules::mov())
                .rule(compile::rules::lbl());

            // Build register-dependent rules
            for register in registers.iter() {
                program
                    .rule(compile::rules::inc_nonzero(&register.name))
                    .rule(compile::rules::inc_zero(&register.name))
                    .rule(compile::rules::dec_zero(&register.name))
                    .rule(compile::rules::dec_one(&register.name))
                    .rule(compile::rules::dec_more(&register.name))
                    .rule(compile::rules::jz_nonzero(&register.name));
            }

            // Build label-dependent rules
            for label in labels.iter() {
                program
                    .rule(compile::rules::bind(&label.name));
            }

            // Build label-register-dependent rules
            for label in labels.iter() {
                for register in registers.iter() {
                    program
                        .rule(compile::rules::jz_zero(&register.name, &label.name));
                }
            }

            program
        };

        println!("// {}\n{:#}", filename, program);
    }
}
