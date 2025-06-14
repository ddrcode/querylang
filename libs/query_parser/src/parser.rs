use pest_derive::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "src/grammar/query.pest"]
pub struct QueryParser;

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
