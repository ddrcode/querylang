use pest::Parser;
use serde::{Deserialize, Serialize};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{error::AppError::{self, ParseError}, parser::{parse_query, Query, QueryParser, Rule}};

#[derive(Deserialize)]
pub struct QueryReq {
    query: String,
}

#[derive(Serialize)]
pub struct QueryResp {
    status: String,
    message: String
}

impl QueryResp {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),
            message: message.to_string()
        }
    }
}

pub async fn handle_query(Json(req): Json<QueryReq>) -> Result<impl IntoResponse, StatusCode> {
    let msg = match parse(&req.query) {
        Ok(query) => QueryResp::new("ok", &format!("You sent: {}", query)),
        Err(err) => QueryResp::new("error", &err.to_string())
    };
    Ok(Json(msg))
}

fn parse(query: &str) -> Result<Query, AppError> {
    let mut parsed = QueryParser::parse(Rule::query, query)?;
    let next = parsed.next().ok_or(ParseError("empty query"))?;
    parse_query(next)
}
