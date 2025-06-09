use crate::error::AppError::{self, ParseError};

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

