use asm::AsmProgram;
use asm::Arg;
use asm::Line;
use asm::Instruction;

pub fn unroll_mov<'a, 'b>(asm: &'b mut AsmProgram<'a>) -> &'b mut AsmProgram<'a> {

    let lines = std::mem::replace(asm.lines_mut(), Vec::new());
    let new = asm.lines_mut();

    for line in lines.into_iter() {
        match line {
            Line::OpLine(ref ins) if ins.op() == "mov" => {
                let (lit, reg) = args!(ins, mov(Arg::Literal, Arg::Register));
                new.push(Line::OpLine(
                    Instruction::with_args("clr", Some(Arg::Register(reg.clone())))
                ));
                for _ in 0..lit.value() {
                    let mut unrolled = Instruction::new("inc");
                    unrolled.argument(Arg::Register(reg.clone()));
                    new.push(Line::OpLine(unrolled));
                }
            }
            _ => new.push(line),
        }
    }

    asm
}
