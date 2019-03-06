use std::iter::repeat;

use attasm::ast::Arg;
use attasm::ast::Instruction;
use attasm::ast::Label;
use attasm::ast::Line;
use attasm::ast::Literal;
use attasm::ast::Program;
use attasm::ast::Register;

pub fn desugar_mov<'a, 'b>(asm: &'b mut Program<'a>) -> &'b mut Program<'a> {
    // change program lines inplace
    let len = asm.lines().len();
    let lines = std::mem::replace(asm.lines_mut(), Vec::with_capacity(len));
    let new = asm.lines_mut();

    // modify lines inplace to replace `mov $x, %r` with `inc %r` instructions
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
                            match lit {
                                Literal::Dec(n) | Literal::Oct(n) | Literal::Hex(n) => {
                                    new.extend(repeat(new_ins.into()).take(*n));
                                }
                                _ => panic!("unsupported: {}", ins),
                            }
                        }
                        _ => new.push(ins.into()),
                    },
                    _ => new.push(ins.into()),
                };
            }
            _ => new.push(line),
        }
    }

    // return reference to program
    asm
}

pub fn impl_cpy<'a, 'b>(asm: &'b mut Program<'a>) -> &'b mut Program<'a> {
    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    // counter for private labels
    let mut i = 0;

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ref ins) if ins.mnemonic() == "cpy" => {
                let mut new_ins: Instruction;
                let mut new_label: Label;

                // extract arguments
                let (src, dst) = args!(ins, mov(Arg::Register, Arg::Register));
                let cpy = Register::new("_cpy");

                // Implement `cpy` using {clr, inc, dec, jnz}:
                // clr(%dst)
                new_ins = Instruction::new("clr");
                new_ins.add_argument(dst.clone());
                new.push(new_ins.into());
                // mov(%src, %_cpy)
                new_ins = Instruction::new("mov");
                new_ins.add_argument(src.clone());
                new_ins.add_argument(cpy.clone());
                new.push(new_ins.into());
                // _cpy_i:
                let labelname = format!("_cpy_{}", {
                    i += 1;
                    i
                });
                new_label = Label::new(labelname.clone());
                new.push(new_label.clone().into());
                // inc %src
                new_ins = Instruction::new("inc");
                new_ins.add_argument(src.clone());
                new.push(new_ins.into());
                // inc %dst
                new_ins = Instruction::new("inc");
                new_ins.add_argument(dst.clone());
                new.push(new_ins.into());
                // dec %_cpy
                new_ins = Instruction::new("dec");
                new_ins.add_argument(cpy.clone());
                new.push(new_ins.into());
                // jnz %_cpy,_cpy_{}
                new_ins = Instruction::new("jnz");
                new_ins.add_argument(cpy.clone());
                new_ins.add_argument(new_label.clone());
                new.push(new_ins.into())
            }
            _ => new.push(line),
        }
    }

    asm
}
