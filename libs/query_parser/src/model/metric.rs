use crate::error::ParseError;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
#[non_exhaustive]
pub enum Metric {
    Volume,
    Max,
    Min,
    Open,
    Close,
    Avg,
}

impl TryFrom<&str> for Metric {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "volume" => Metric::Volume,
            "max" => Metric::Max,
            "min" => Metric::Min,
            "open" => Metric::Open,
            "close" => Metric::Close,
            "avg" => Metric::Avg,
            other => {
                return Err(ParseError::InvalidValue(
                    other.to_string().into(),
                    "metric".into(),
                ));
            }
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
