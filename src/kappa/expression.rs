//!

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::pattern::Pattern;

pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divided,
    Power,
    Mod
}

pub enum UnaryOp {
    Log,
    Exp,
    Sin,
    Cos,
    Tan,
    Sqrt,
}

pub enum Reserved {
    Pi,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlgebraicExpression {
    Float(f64),
    // TODO
    // Reserved(Reserved),
    // Variable(String),
    // BinOp(Box<AlgebraicExpression>, BinaryOperator, Box<AlgebraicExpression>),
    // UnOp(UnaryOp, AlgebraicExpression),
    // Max(AlgebraicExpression, AlgebraicExpression),
    // Min(AlgebraicExpression, AlgebraicExpression),
    // Conditional(AlgebraicExpression, AlgebraicExpression, AlgebraicExpression)
    Occurrences(Pattern)
}

impl Display for AlgebraicExpression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            AlgebraicExpression::Float(value) => value.fmt(f),
            AlgebraicExpression::Occurrences(pattern) =>
                f.write_char('|')
                    .and(pattern.fmt(f))
                    .and(f.write_char('|'))
        }
    }
}
