use crate::error::AppError::{self, ParseError};
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
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
            other => return Err(ParseError(format!("Unrecognised time unit: {other}"))),
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
