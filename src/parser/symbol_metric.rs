use crate::error::AppError::{self, ParseError};

use super::Metric;

#[derive(Debug, PartialEq)]
pub struct SymbolMetric {
    pub symbol: String,
    pub metric: Metric,
}

impl SymbolMetric {
    pub fn new(symbol: &str, metric: Metric) -> Self {
        Self {
            symbol: symbol.to_string(),
            metric,
        }
    }
}
