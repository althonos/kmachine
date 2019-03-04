#[macro_use]
mod macros;

#[derive(Parser)]
#[grammar = "parser/att.pest"]
pub struct Parser;

// impl AsmParser for AttParser {
//     fn parse_asm(s: &str) -> AsmProgram<'_> {
//         AttParser::parse(Rule::program, s)
//             .map(|pairs| {
//                 pairs
//                     .into_iter()
//                     .flat_map(|pair| match pair.as_rule() {
//                         Rule::EOI => None,
//                         Rule::labeldecl => {
//                             let ident = pair.into_inner().next().unwrap().as_str();
//                             Some(Line::from(Label::new(ident.to_string())))
//                         }
//                         Rule::instruction => {
//                             let mut inner = pair.into_inner();
//                             let opname = inner.next().unwrap().as_str();
//                             let args = inner.flat_map(|p| p.into_inner().next()).map(|pair| {
//                                 match pair.as_rule() {
//                                     Rule::register => {
//                                         let name = pair.into_inner().next().unwrap();
//                                         Arg::Register(Register::new(name.as_str()))
//                                     }
//                                     Rule::literal => {
//                                         let digits = pair.into_inner().next().unwrap();
//                                         let val = match digits.as_rule() {
//                                             Rule::hexnumber => {
//                                                 usize::from_str_radix(digits.as_str(), 16).unwrap()
//                                             }
//                                             Rule::number => {
//                                                 usize::from_str_radix(digits.as_str(), 10).unwrap()
//                                             }
//                                             _ => unreachable!(),
//                                         };
//                                         Arg::Literal(Literal::new(val))
//                                     }
//                                     Rule::label => {
//                                         let name = pair.into_inner().next().unwrap();
//                                         Arg::Label(Label::new(name.as_str()))
//                                     }
//                                     _ => unreachable!(),
//                                 }
//                             });
//                             Some(Line::from(Instruction::with_args(opname, args)))
//                         }
//                         _ => unreachable!(),
//                     })
//                     .collect()
//             })
//             .unwrap_or_else(|e| panic!("{}", e))
//     }
// }
