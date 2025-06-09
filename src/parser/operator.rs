use crate::error::AppError::{self, ParseError};

#[derive(Debug, PartialEq)]
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

