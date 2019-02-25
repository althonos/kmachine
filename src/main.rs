#[macro_use]
extern crate pest_derive;

extern crate pest;

pub mod asm;

use std::collections::HashSet;
use std::io::Read;

use self::asm::att::AttParser;
use self::asm::AsmParser;
use self::asm::Label;
use self::asm::Line;
use self::asm::Op;
use self::asm::Register;

fn main() {
    for filename in std::env::args().skip(1) {
        println!("{}\n\n", filename);

        let mut file = std::fs::File::open(filename).unwrap();
        let mut program = String::new();
        file.read_to_string(&mut program).unwrap();

        // Parse the ASM program
        let asm = AttParser::parse_asm(&program);

        // Collect all registers used in the program.
        let registers: HashSet<&Register> = asm
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
        let labels: HashSet<&Label> = asm
            .iter()
            .flat_map(|line| match line {
                Line::LabelLine(l) => Some(l),
                Line::OpLine(_) => None,
            })
            .collect();
    }
}
