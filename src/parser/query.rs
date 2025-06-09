use std::fmt;
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

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expr: Vec<String> = self.expressions.iter().map(|e| e.to_string()).collect();
        write!(f, "GET {} FOR {} STEP {}", expr.join(", "), self.for_clause, self.step)
    }
}
