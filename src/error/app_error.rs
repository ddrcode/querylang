use pest;
use reqwest;
use thiserror::Error;

use crate::parser::Rule;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {

    #[error("Query parse error: {0}")]
    ParseError(String),

    #[error("GraphQL error: {0}")]
    GQLError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Data processing error: {0}")]
    DataError(String)
}

impl From<pest::error::Error<Rule>> for AppError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}
