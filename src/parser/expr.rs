use std::fmt;
use super::{Operator, SymbolMetric};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Data(SymbolMetric),
    Value(u32),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;
        match self {
            Binary(left, op, right) => write!(f, "{} {} {}", left, op, right),
            Data(symbol) => write!(f, "{}", symbol),
            Value(val) => write!(f, "{}", val)
        }
    }
}
