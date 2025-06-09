use crate::error::AppError::{self, ParseError};

#[derive(Debug)]
pub struct Query {
    pub expressions: Vec<Expr>,
    pub for_clause: TimeRange,
    pub step: TimeRange,
}

impl Query {
    pub fn new(expressions: Vec<Expr>, for_clause: TimeRange, step_clause: TimeRange) -> Self {
        Self {
            expressions,
            for_clause,
            step: step_clause,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Data(SymbolMetric),
    Value(u32),
}

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

#[derive(Debug, PartialEq)]
pub enum Metric {
    Volume,
    Max,
    Min,
    Open,
    Close,
    Avg
}

impl TryFrom<&str> for Metric {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "volume" => Metric::Volume,
            "max" => Metric::Max,
            "min" => Metric::Min,
            "open" => Metric::Open,
            "close" => Metric::Close,
            "avg" => Metric::Avg,
            _ => return Err(ParseError("Unknown metric"))
        };
        Ok(val)
    }
}

#[derive(Debug, PartialEq)]
pub struct SymbolMetric {
    pub symbol: String,
    pub metric: Metric,
}

impl SymbolMetric {
    pub fn new(symbol: &str, metric: Metric) -> Self {
        Self {
            symbol: symbol.to_string(),
            metric
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimeUnit {
    Hour,
    Day,
}

impl TryFrom<&str> for TimeUnit {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "day" => TimeUnit::Day,
            "days" => TimeUnit::Day,
            "hour" => TimeUnit::Hour,
            "hours" => TimeUnit::Hour,
            _ => return Err(ParseError("Incorrect time unit")),
        };
        Ok(val)
    }
}

#[derive(Debug)]
pub struct TimeRange {
    pub value: u32,
    pub unit: TimeUnit,
}

impl TimeRange {
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }
}

impl TryFrom<(&str, &str)> for TimeRange {
    type Error = AppError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            value: String::from(value.0)
                .parse()
                .map_err(|_| ParseError("Error parsing value in TimeRange"))?,
            unit: TimeUnit::try_from(value.1)?,
        })
    }
}
