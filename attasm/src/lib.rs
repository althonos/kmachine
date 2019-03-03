//! AT&T Assembler Syntax Tree.
#![feature(try_from)]

#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
pub mod parser;
pub mod ast;
