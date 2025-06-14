use crate::error::ParseError;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn opfn(&self) -> fn(f32, f32) -> f32 {
        use Operator::*;
        match self {
            Add => |a, b| a + b,
            Sub => |a, b| a - b,
            Mul => |a, b| a * b,
            Div => |a, b| a / b,
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let op = match value {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            op => return Err(ParseError::InvalidValue(op.into(), "operator".into())),
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
            Div => "/",
        };
        write!(f, "{}", op)
    }
}
