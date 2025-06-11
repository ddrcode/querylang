use std::fmt;
use super::Metric;

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolMetric {
    symbol: String,
    metric: Metric,
}

impl SymbolMetric {
    pub fn new(symbol: &str, metric: Metric) -> Self {
        Self {
            symbol: symbol.to_string(),
            metric,
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn metric(&self) -> Metric {
        self.metric
    }
}

impl fmt::Display for SymbolMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.symbol, self.metric)
    }
}
