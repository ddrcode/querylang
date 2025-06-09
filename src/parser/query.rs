use super::{Expr, TimeRange};

#[derive(Debug)]
pub struct Query {
    pub expressions: Vec<Expr>,
    pub for_clause: TimeRange,
    pub step: TimeRange,
}

impl Query {
    pub fn new(expressions: Vec<Expr>, for_clause: TimeRange, step_clause: TimeRange) -> Self {
        Self {
            expressions,
            for_clause,
            step: step_clause,
        }
    }
}

