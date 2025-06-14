use thiserror;

use super::parser::Rule;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Missing pair: {0}")]
    MissingPair(&'static str),

    #[error("Invalid rule. Expected {0}, but found {1}")]
    InvalidRule(String, String),

    #[error("Invalid value: {0} for {1} rule ")]
    InvalidValue(String, String),

    #[error("Internal parser error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}


