use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct DateRange {
    from: SystemTime,
    to: SystemTime
}


impl DateRange {
    pub fn new(from: SystemTime, to: SystemTime) -> Self {
        Self { from, to }
    }

    pub fn from_now(delta: Duration) -> Self {
        let to = SystemTime::now();
        let from = to.checked_sub(delta).unwrap();
        Self { from, to }
    }

    pub fn from(&self) -> SystemTime {
        self.from
    }

    pub fn to(&self) -> SystemTime {
        self.to
    }
}
