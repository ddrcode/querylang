use super::TimeUnit;
use crate::error::AppError::{self, ParseError};
use std::{fmt, time::Duration};

#[derive(Debug)]
pub struct TimeSpec {
    value: u32,
    unit: TimeUnit,
}

impl TimeSpec {
    pub fn new(value: u32, unit: TimeUnit) -> Self {
        Self { value, unit }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit
    }

    pub fn to_seconds(&self) -> u64 {
        match self.unit {
            TimeUnit::Hour => 3600 * self.value as u64,
            TimeUnit::Day => 86400 * self.value as u64,
        }
    }
}

impl TryFrom<(&str, &str)> for TimeSpec {
    type Error = AppError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let spec = Self {
            value: String::from(value.0)
                .parse()
                .map_err(|_| ParseError("Error parsing value in TimeRange".to_string()))?,
            unit: TimeUnit::try_from(value.1)?,
        };
        if spec.value < 1 {
            return Err(ParseError("Time value must be > 0".to_string()));
        }
        Ok(spec)
    }
}

impl fmt::Display for TimeSpec {
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

impl From<&TimeSpec> for Duration {
    fn from(tr: &TimeSpec) -> Self {
        match tr.unit {
            TimeUnit::Hour => Duration::from_secs(tr.to_seconds()),
            TimeUnit::Day => Duration::from_secs(tr.to_seconds()),
        }
    }
}
