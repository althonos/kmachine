#![allow(unused_macros, unused_imports)]

#[macro_use]
extern crate kappa;

extern crate asm;
extern crate indexmap;

#[macro_use]
mod macros;

mod agents;
mod inits;
mod rules;
mod transformation;

use std::borrow::Cow;
use std::io::Read;

use asm::Arg;
use asm::AsmParser;
use asm::AttParser;
use asm::Label;
use asm::Line;
use asm::Register;
use indexmap::IndexSet;
use kappa::Agent;
use kappa::AlgebraicExpression;
use kappa::Init;
use kappa::KappaProgram;
use kappa::Observable;
use kappa::Pattern;
use kappa::Site;

fn main() {
    for filename in std::env::args().skip(1) {
        let mut file = std::fs::File::open(&filename).unwrap();
        let mut program = String::new();
        file.read_to_string(&mut program).unwrap();

        // Parse the ASM program and run some program transforms
        let mut asm = AttParser::parse_asm(&program);
        transformation::unroll_mov(&mut asm);

        // Collect all registers used in the program.
        let registers: IndexSet<_> = asm.registers().into_iter().map(|r| &r.name).collect();

        // Collect all labels declared in the program.
        let labels: IndexSet<_> = asm.labels().into_iter().map(|l| &l.name).collect();

        // Compile the CM program into a Kappa source
        let mut program = KappaProgram::new();

        // Build agents
        program
            // counter units
            .agent(agents::unit(&registers))
            // machine
            .agent(agents::machine(&registers, &labels))
            // instructions
            .agent(agents::prog())
            // pseudo-operands
            .agent(agents::lbl(&labels))
            .agent(agents::clr(&registers))
            .agent(agents::inc(&registers))
            .agent(agents::dec(&registers))
            .agent(agents::jz(&registers, &labels))
            .agent(agents::jmp(&labels));

        // Build static rules
        program
            .rule(rules::mov())
            .rule(rules::lbl());
        // Build register-dependent rules
        for register in registers.iter() {
            program
                .rule(rules::inc_nonzero(register))
                .rule(rules::inc_zero(register))
                .rule(rules::dec_zero(register))
                .rule(rules::dec_one(register))
                .rule(rules::dec_more(register))
                .rule(rules::jz_nonzero(register))
                .rule(rules::clr_zero(register))
                .rule(rules::clr_one(register))
                .rule(rules::clr_more(register));
        }
        // Build label-dependent rules
        for label in labels.iter() {
            program.rule(rules::bind(label))
                .rule(rules::jmp(label));
        }
        // Build label-register-dependent rules
        for label in labels.iter() {
            for register in registers.iter() {
                program.rule(rules::jz_zero(register, label));
            }
        }

        // Build static init
        program
            .init(inits::units(100))
            .init(inits::program(&asm));

        // Build observables
        for register in registers.iter() {
            let r: &str = register.as_ref();
            let pattern = Pattern::from(vec![agent!(UNIT(r{?r}))]);
            let obs = Observable::new(r, AlgebraicExpression::Occurrences(pattern));
            program.observable(obs);
        }

        println!("// {}\n{:#}", filename, program);
    }
}
