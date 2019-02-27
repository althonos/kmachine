//! `asm` parser for the AT&T syntax.

use std::collections::VecDeque;

use ::pest::Parser;

use super::AsmParser;
use super::AsmProgram;
use super::Label;
use super::Line;
use super::Op;
use super::Register;

#[derive(Parser)]
#[grammar = "asm/att.pest"]
pub struct AttParser;

impl AsmParser for AttParser {
    fn parse_asm<'a>(s: &'a str) -> AsmProgram<'a> {
        AttParser::parse(Rule::program, s)
            .map(|pairs| {
                pairs
                    .into_iter()
                    .map(|pair| match pair.as_rule() {
                        Rule::label => {
                            let ident = pair.into_inner().next().unwrap().as_str();
                            Line::from(Label::new(ident.to_string()))
                        }
                        Rule::instruction => {
                            let mut inner = pair.into_inner();
                            let opname = inner.next().unwrap().as_str();

                            let mut args: VecDeque<_> = inner.map(|r| r.into_inner()).collect();
                            let op = match opname {
                                "inc" => {
                                    let register = args.pop_front().unwrap().as_str();
                                    Op::Inc(Register::new(register.to_string()))
                                }
                                "dec" => {
                                    let register = args.pop_front().unwrap().as_str();
                                    Op::Dec(Register::new(register.to_string()))
                                }
                                "jz" => {
                                    let register = args.pop_front().unwrap().as_str();
                                    let label = args.pop_front().unwrap().as_str();
                                    Op::Jz(
                                        Register::new(register.to_string()),
                                        Label::new(label.to_string()),
                                    )
                                }
                                "clr" => {
                                    let register = args.pop_front().unwrap().as_str();
                                    Op::Clr(Register::new(register.to_string()))
                                }
                                name => panic!("unknown instruction: {:?}", name),
                            };

                            if !args.is_empty() {
                                panic!("unused arguments for instruction `{:?}`: {:?}", op, args);
                            }

                            Line::from(op)
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
