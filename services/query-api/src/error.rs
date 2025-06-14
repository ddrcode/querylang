use reqwest;
use thiserror::Error;
use query_parser;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {

    #[error("Query parse error: {0}")]
    ParseError(#[from] query_parser::ParseError),

    #[error("GraphQL error: {0}")]
    GQLError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Data processing error: {0}")]
    DataError(String)
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}
