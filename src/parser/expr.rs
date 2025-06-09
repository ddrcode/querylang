use super::{Operator, SymbolMetric};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Data(SymbolMetric),
    Value(u32),
}

