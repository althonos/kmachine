#[macro_use]
extern crate pest_derive;

extern crate indexmap;
extern crate pest;

pub mod asm;
pub mod kappa;

use std::collections::HashSet;
use std::io::Read;

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

        // UNIT agent
        let mut agent_unit = Agent::new("UNIT");
        agent_unit
            .site({
                let mut site = Site::new("prev");
                site.binding("UNIT", "next");
                for register in registers.iter() {
                    site.binding("MACHINE", register.name.as_str());
                }
                site
            })
            .site({
                let mut site = Site::new("next");
                site.binding("UNIT", "prev");
                site
            })
            .site({
                let mut site = Site::new("r");
                site.state("_none");
                for ref register in registers {
                    site.state(register.name.as_str());
                }
                site
            });

        // MACHINE agent
        let mut agent_machine = Agent::new("MACHINE");


        let mut kappa = KappaProgram::new().agent(agent_unit);

        // println!("
        //     %mod: |UNIT(prev[.])| = 0 do $ADD 1 UNIT();
        // ")
    }
}
