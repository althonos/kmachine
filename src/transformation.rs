use asm::AsmProgram;
use asm::Arg;
use asm::Line;
use asm::Instruction;


pub fn desugar_mov<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {

    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ins) => {
                let new_ins = match ins.mnemonic() {
                    "mov" => Instruction::with_args(
                        match ins.arguments().first() {
                            Some(Arg::Literal(_)) => "movl",
                            Some(Arg::Register(_)) => "mov",
                            Some(a) => panic!("invalid argument #1 for instruction 'mov': {:?}", a),
                            None => panic!("missing argument #1 for instruction 'mov'"),
                        },
                        ins.arguments().iter().cloned(),
                    ),
                    _ => ins,
                };
                new.push(new_ins.into());
            }
            _ => new.push(line),
        }
    }

    asm
}


pub fn impl_movl<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {

    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ref ins) if ins.mnemonic() == "movl" => {
                let (lit, reg) = args!(ins, mov(Arg::Literal, Arg::Register));
                new.push(Line::OpLine(
                    Instruction::with_args("clr", Some(Arg::Register(reg.clone())))
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
}


pub fn impl_copy<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {

    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ref ins) if ins.mnemonic() == "cpy" => {

                let (lit, reg) = args!(ins, mov(Arg::Literal, Arg::Register));
                new.push(Line::OpLine(
                    Instruction::with_args("clr", Some(Arg::Register(reg.clone())))
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
}
