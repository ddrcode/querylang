use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/query.pest"]
pub struct QueryParser;
