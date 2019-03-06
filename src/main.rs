#![feature(try_from)]
#![allow(unused_macros)]

#[macro_use]
extern crate kappa;

extern crate attasm;
extern crate indexmap;

#[macro_use]
mod macros;

mod agents;
mod inits;
mod rules;
mod transformation;

use std::convert::TryFrom;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use attasm::ast::Program as AsmProgram;
use indexmap::IndexSet;
use kappa::AlgebraicExpression;
use kappa::KappaProgram;
use kappa::Observable;
use kappa::Pattern;

fn main() {
    for filename in std::env::args_os().skip(1).map(PathBuf::from) {

        // Read the ASM program
        let mut file = std::fs::File::open(&filename).unwrap();
        let mut program = String::new();
        file.read_to_string(&mut program).unwrap();

        // Parse the ASM code
        let mut asm = AsmProgram::try_from(&program as &str).unwrap();

        // Collect all registers declared in the original program
        let public_registers: IndexSet<String> = asm.registers().into_iter().map(|r| r.name().to_string()).collect();

        // Run program transformations
        transformation::desugar_mov(&mut asm);
        transformation::impl_cpy(&mut asm);

        // Write the transformed ASM program.
        let mut out_asm = std::fs::File::create(filename.with_extension("desugared.S")).unwrap();
        write!(out_asm, "{}", asm);

        // Collect all registers used in the program.
        let registers: IndexSet<_> = asm.registers().iter().map(|r| r.name()).collect();

        // Collect all labels declared in the program.
        let labels: IndexSet<_> = asm.labels().iter().map(|l| l.name()).collect();

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
            .agent(agents::instructions::clr(&registers))
            .agent(agents::instructions::dec(&registers))
            .agent(agents::instructions::inc(&registers))
            .agent(agents::instructions::jmp(&labels))
            .agent(agents::instructions::jz(&registers, &labels))
            .agent(agents::instructions::jnz(&registers, &labels))
            .agent(agents::instructions::lbl(&labels))
            .agent(agents::instructions::mov(&registers));

        // Build static rules
        program
            .rule(rules::next())
            .rule(rules::reset_units())
            .rule(rules::instructions::lbl());
        // Build register-dependent rules
        for register in registers.iter() {
            program
                .rule(rules::instructions::inc_nonzero(register))
                .rule(rules::instructions::inc_zero(register))
                .rule(rules::instructions::dec_zero(register))
                .rule(rules::instructions::dec_one(register))
                .rule(rules::instructions::dec_more(register))
                .rule(rules::instructions::jz_nonzero(register))
                .rule(rules::instructions::jnz_zero(register))
                .rule(rules::instructions::clr_zero(register))
                .rule(rules::instructions::clr_nonzero(register));
        }
        // Build label-dependent rules
        for label in labels.iter() {
            program
                .rule(rules::bind(label))
                .rule(rules::instructions::jmp(label));
        }
        // Build label-register-dependent rules
        for label in labels.iter() {
            for register in registers.iter() {
                program
                    .rule(rules::instructions::jnz_nonzero(register, label))
                    .rule(rules::instructions::jz_zero(register, label));
            }
        }
        // Build register-register-dependent rules
        for src in registers.iter() {
            for dst in registers.iter() {
                if src != dst {
                    program
                        .rule(rules::relabel_units(src, dst))
                        .rule(rules::instructions::mov_zero(src, dst))
                        .rule(rules::instructions::mov_nonzero(src, dst));
                }
            }
        }

        // Build static init
        program.init(inits::units(100)).init(inits::program(&asm));

        // Build observables
        for register in public_registers.iter() {
            let r: &str = register.as_ref();
            let pattern = Pattern::from(vec![agent!(UNIT(r{?r}))]);
            let obs = Observable::new(r, AlgebraicExpression::Occurrences(pattern));
            program.observable(obs);
        }

        // Write the Kappa program.
        let mut out_ka = std::fs::File::create(filename.with_extension("ka")).unwrap();
        write!(out_ka, "// {}\n{:#}\n", filename.to_string_lossy(), program);
    }
}
