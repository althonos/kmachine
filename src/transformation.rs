use std::iter::repeat;

use asm::Arg;
use asm::AsmProgram;
use asm::Instruction;
use asm::Line;

pub fn desugar_mov<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {
    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ins) => {
                match ins.mnemonic() {
                    "mov" => match ins.arguments().first() {
                        Some(Arg::Literal(_)) => {
                            let (lit, reg) = args!(ins, mov(Arg::Literal, Arg::Register));
                            // clr(r)
                            let mut new_ins = Instruction::new("clr");
                            new_ins.add_argument(reg.clone());
                            new.push(Line::OpLine(new_ins.clone()));
                            // inc(r)
                            new_ins.set_mnemonic("inc");
                            new.extend(repeat(new_ins.into()).take(lit.value()));
                        }
                        _ => new.push(ins.into()),
                    }
                    _ => new.push(ins.into()),
                };
            }
            _ => new.push(line),
        }
    }

    asm
}

/*
pub fn impl_cpy<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {
    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ref ins) if ins.mnemonic() == "cpy" => {
                let (src, dst) = args!(ins, mov(Arg::Register, Arg::Register));

                // new.push(Line::OpLine(Instruction::with_args(
                //     "clr",
                //     Some(Arg::Register(reg.clone())),
                // )));

                new.push(Line::OpLine(Instruction::with_args(
                    "mov",
                ));



                for _ in 0..lit.value() {
                    let mut unrolled = Instruction::new("inc");
                    unrolled.add_argument(Arg::Register(reg.clone()));
                    new.push(Line::OpLine(unrolled));
                }
            }
            _ => new.push(line),
        }
    }

    asm
}*/
