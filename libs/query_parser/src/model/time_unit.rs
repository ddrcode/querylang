use crate::error::ParseError;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TimeUnit {
    Hour,
    Day,
}

impl TryFrom<&str> for TimeUnit {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "day" => TimeUnit::Day,
            "days" => TimeUnit::Day,
            "hour" => TimeUnit::Hour,
            "hours" => TimeUnit::Hour,
            other => {
                return Err(ParseError::InvalidValue(
                    other.to_string().into(),
                    "time_unit".into(),
                ));
            }
        };
        Ok(val)
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TimeUnit::*;
        write!(
            f,
            "{}",
            match self {
                Hour => "hour",
                Day => "day",
            }
        )
    }
}
