use axum::{http::StatusCode, response::IntoResponse, Json};
use pest::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    config,
    error::AppError::{self, ParseError},
    parser::{parse_query, Query, QueryParser, Rule},
    query_engine::{GraphQLClient, QueryPlan},
};

#[derive(Deserialize)]
pub struct QueryReq {
    query: String,
}

#[derive(Serialize)]
pub struct QueryResp {
    status: String,
    message: String,
}

impl QueryResp {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}

pub async fn handle_query(Json(req): Json<QueryReq>) -> Result<impl IntoResponse, StatusCode> {
    let resp = match execute_query(&req.query).await {
        Ok(data) => QueryResp::new("ok", &data),
        Err(err) => QueryResp::new("error", &err.to_string()),
    };
    Ok(Json(resp))
}

async fn execute_query(query_str: &str) -> Result<String, AppError> {
    let parsed_query = parse(query_str)?;
    let gql_client = GraphQLClient::new(config::GRAPHQL_SERVER);
    let plan = QueryPlan::from(&parsed_query);

    gql_client.fetch_metrics().await
}

fn parse(query: &str) -> Result<Query, AppError> {
    let mut parsed = QueryParser::parse(Rule::query, query)?;
    let next = parsed.next().ok_or(ParseError("empty query"))?;
    parse_query(next)
}
