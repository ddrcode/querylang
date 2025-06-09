use std::fmt;
use crate::error::AppError::{self, ParseError};

#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
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

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Metric::*;
        let val = match self {
            Volume => "volume",
            Max => "max",
            Min => "min",
            Open => "open",
            Close => "close",
            Avg => "avg",
        };
        write!(f, "{}", val)
    }
}
