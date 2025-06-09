use std::fmt;
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

impl fmt::Display for SymbolMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.symbol, self.metric)
    }
}
