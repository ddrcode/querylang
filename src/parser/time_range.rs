use super::TimeUnit;
use crate::error::AppError::{self, ParseError};
use std::fmt;

#[derive(Debug)]
pub struct TimeRange {
    value: u32,
    unit: TimeUnit,
}

impl TimeRange {
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit
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

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.value,
            self.unit,
            if self.value != 1 { "s" } else { "" }
        )
    }
}
