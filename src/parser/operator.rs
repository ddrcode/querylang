use std::fmt;
use crate::error::AppError::{self, ParseError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<&str> for Operator {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let op = match value {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => return Err(ParseError("Unknown operator")),
        };
        Ok(op)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operator::*;
        let op = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/"
        };
        write!(f, "{}", op)
    }
}
