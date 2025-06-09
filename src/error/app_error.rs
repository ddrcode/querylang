use pest::error::Error;
use thiserror::Error;

use crate::parser::Rule;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {

    #[error("Query parse error: {0}")]
    ParseError(&'static str),

    #[error("Parser error: {0}")]
    PestError(String)
}

impl From<Error<Rule>> for AppError {
    fn from(value: Error<Rule>) -> Self {
        AppError::PestError(value.to_string())
    }
}
