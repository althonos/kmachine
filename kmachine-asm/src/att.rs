//! `asm` parser for the AT&T syntax.

use std::collections::VecDeque;
use std::str::FromStr;

use ::pest::Parser;

use super::Arg;
use super::AsmParser;
use super::AsmProgram;
use super::Instruction;
use super::Label;
use super::Line;
use super::Literal;
use super::Register;

#[derive(Parser)]
#[grammar = "att.pest"]
pub struct AttParser;

impl AsmParser for AttParser {
    fn parse_asm<'a>(s: &'a str) -> AsmProgram<'a> {
        AttParser::parse(Rule::program, s)
            .map(|pairs| {
                pairs
                    .into_iter()
                    .map(|pair| match pair.as_rule() {
                        Rule::labeldecl => {
                            let ident = pair.into_inner().next().unwrap().as_str();
                            Line::from(Label::new(ident.to_string()))
                        }
                        Rule::instruction => {
                            let mut inner = pair.into_inner();
                            let opname = inner.next().unwrap().as_str();
                            let args = inner.flat_map(|p| p.into_inner().next()).map(|pair| {
                                match pair.as_rule() {
                                    Rule::register => {
                                        let name = pair.into_inner().next().unwrap();
                                        Arg::Register(Register::new(name.as_str()))
                                    }
                                    Rule::literal => {
                                        let repr = pair.into_inner().next().unwrap();
                                        let val = usize::from_str(repr.as_str()).unwrap();
                                        Arg::Literal(Literal::new(val))
                                    }
                                    Rule::label => {
                                        let name = pair.into_inner().next().unwrap();
                                        Arg::Label(Label::new(name.as_str()))
                                    }
                                    _ => unreachable!(),
                                }
                            });
                            Line::from(Instruction::with_args(opname, args))
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
