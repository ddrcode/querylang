use std::{collections::HashMap, time::Duration};

use crate::parser::{Expr, Query, SymbolMetric};

use super::DataTarget;

#[derive(Debug)]
pub struct QueryPlan {
    targets: Vec<DataTarget>,
    step: Duration,
}

impl QueryPlan {
    pub fn new(targets: Vec<DataTarget>, step: Duration) -> Self {
        Self { targets, step }
    }

    pub fn targets(&self) -> impl Iterator<Item = &DataTarget> {
        self.targets.iter()
    }

    pub fn step(&self) -> Duration {
        self.step
    }
}

impl From<&Query> for QueryPlan {
    fn from(query: &Query) -> Self {
        let mut targets: HashMap<String, DataTarget> = HashMap::with_capacity(5);

        let mut symbols: Vec<&SymbolMetric> = Vec::new();
        for expr in query.expressions() {
            collect_symbols(&expr, &mut symbols);
        }

        for sm in symbols {
            let target = targets
                .entry(sm.symbol.clone())
                .or_insert_with(|| DataTarget::new(&sm.symbol));
            target.add_metric(sm.metric);
        }

        QueryPlan {
            targets: Vec::from_iter(targets.values().cloned()),
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
