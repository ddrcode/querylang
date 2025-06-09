use super::{Expr, TimeSpec};
use std::fmt;

#[derive(Debug)]
pub struct Query {
    expressions: Vec<Expr>,
    for_clause: TimeSpec,
    step: TimeSpec,
}

impl Query {
    pub fn new(expressions: Vec<Expr>, for_clause: TimeSpec, step_clause: TimeSpec) -> Self {
        Self {
            expressions,
            for_clause,
            step: step_clause,
        }
    }

    pub fn expressions(&self) -> &Vec<Expr> {
        &self.expressions
    }

    pub fn for_clause(&self) -> &TimeSpec {
        &self.for_clause
    }

    pub fn step(&self) -> &TimeSpec {
        &self.step
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expr: Vec<String> = self.expressions.iter().map(|e| e.to_string()).collect();
        write!(
            f,
            "GET {} FOR {} STEP {}",
            expr.join(", "),
            self.for_clause,
            self.step
        )
    }
}
