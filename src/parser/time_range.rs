use crate::error::AppError::{self, ParseError};

use super::TimeUnit;

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
