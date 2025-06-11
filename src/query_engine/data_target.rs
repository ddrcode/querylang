use std::collections::HashSet;

use crate::domain::Metric;

#[derive(Debug, Clone)]
pub struct DataTarget {
    symbol: String,
    metrics: HashSet<Metric>,
}

impl DataTarget {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            metrics: HashSet::new()
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn metrics(&self) -> impl Iterator<Item = &Metric> {
        self.metrics.iter()
    }

    pub fn add_metric(&mut self, metric: Metric) -> bool {
        let contains = self.metrics.contains(&metric);
        if !contains {
            self.metrics.insert(metric);
        }
        contains
    }
}

