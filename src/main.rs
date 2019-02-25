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

        // Compile Kappa program
        let mut program = KappaProgram::new();
        program
            // counter units
            .agent(compile::agents::unit(&registers))
            // machine
            .agent(compile::agents::machine(&registers))
            // instructions
            .agent(compile::agents::prog())
            // pseudo-operands
            .agent(compile::agents::inc(&registers))
            .agent(compile::agents::dec(&registers))
            .agent(compile::agents::jz(&registers, &labels));

        program
            .rule(compile::rules::mov());

        for register in registers.iter() {
            program
                .rule(compile::rules::inc_nonzero(register))
                .rule(compile::rules::inc_zero(register));
        }


        // program.rule({
        //     let mut mov = Rule::with_name("move", 1);
        //     mov.agent(
        //
        //     )
        // });



        println!("// {}\n{:#}", filename, program);
    }
}
