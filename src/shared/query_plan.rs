use std::{collections::HashMap, time::Duration};

use crate::domain::{Expr, Query, SymbolMetric};

use super::TargetMetrics;

#[derive(Debug)]
pub struct QueryPlan {
    targets: Vec<TargetMetrics>,
    range: Duration,
    step: Duration,
}

impl QueryPlan {
    pub fn new(targets: Vec<TargetMetrics>, range: Duration, step: Duration) -> Self {
        Self { targets, range, step }
    }

    pub fn targets(&self) -> impl Iterator<Item = &TargetMetrics> {
        self.targets.iter()
    }

    pub fn range(&self) -> Duration {
        self.range
    }

    pub fn step(&self) -> Duration {
        self.step
    }
}

impl From<&Query> for QueryPlan {
    fn from(query: &Query) -> Self {
        let mut targets: HashMap<String, TargetMetrics> = HashMap::with_capacity(5);

        let mut symbols: Vec<&SymbolMetric> = Vec::new();
        for expr in query.expressions() {
            collect_symbols(&expr, &mut symbols);
        }

        for sm in symbols {
            let target = targets
                .entry(sm.symbol().to_string())
                .or_insert_with(|| TargetMetrics::new(sm.symbol()));
            target.add_metric(sm.metric());
        }

        QueryPlan {
            targets: Vec::from_iter(targets.values().cloned()),
            range: Duration::from(query.for_clause()),
            step: Duration::from(query.step()),
        }
    }
}

fn collect_symbols<'a>(expr: &'a Expr, acc: &mut Vec<&'a SymbolMetric>) {
    match expr {
        Expr::Data(symbol) => acc.push(symbol),
        Expr::Binary(left, _op, right) => {
            collect_symbols(left, acc);
            collect_symbols(right, acc);
        }
        _ => {}
    }
}
