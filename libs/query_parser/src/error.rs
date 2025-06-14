use std::borrow::Cow;

use thiserror;

use super::parser::Rule;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Missing pair: {0}")]
    MissingPair(Cow<'static, str>),

    #[error("Invalid rule. Expected {0}, but found {1}")]
    InvalidRule(Cow<'static, str>, Cow<'static, str>),

    #[error("Invalid value: {0} for {1} rule ")]
    InvalidValue(Cow<'static, str>, Cow<'static, str>),

    #[error("Internal parser error: {0}")]
    Internal(#[from] pest::error::Error<Rule>),
}


