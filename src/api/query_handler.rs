use axum::{
    Extension, Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use pest::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    adapter::parser::{QueryParser, Rule, parse_query},
    domain::{Query, Table},
    error::AppError::{self, ParseError},
    query_engine::fetch_all_query_metrics,
    service::QueryService,
    shared::QueryPlan,
};

#[derive(Deserialize)]
pub struct QueryReq {
    query: String,
    #[serde(default = "default_format")]
    format: OutputFormat,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Text,
}

fn default_format() -> OutputFormat {
    OutputFormat::Text
}

#[derive(Serialize)]
pub struct QueryError {
    status: &'static str,
    message: String,
}

pub enum QueryResultResponse {
    Json(Json<Table>),
    Text(String),
    JsonError(StatusCode, Json<QueryError>),
    TextError(StatusCode, String),
}

impl IntoResponse for QueryResultResponse {
    fn into_response(self) -> Response {
        match self {
            QueryResultResponse::Json(table) => table.into_response(),
            QueryResultResponse::Text(txt) => {
                ([(header::CONTENT_TYPE, "text/plain")], txt).into_response()
            }
            QueryResultResponse::JsonError(code, err) => (code, err).into_response(),
            QueryResultResponse::TextError(code, msg) => {
                (code, [(header::CONTENT_TYPE, "text/plain")], msg).into_response()
            }
        }
    }
}

pub async fn handle_query(
    Extension(service): Extension<QueryService>,
    Json(req): Json<QueryReq>,
) -> impl IntoResponse {
    match execute_query(&req.query, &service).await {
        Ok(table) => {
            let body = match req.format {
                OutputFormat::Text => QueryResultResponse::Text(table.to_string()),
                OutputFormat::Json => QueryResultResponse::Json(Json(table)),
            };
            (StatusCode::OK, body).into_response()
        }

        Err(err) => {
            let status = match err {
                AppError::ParseError(_) => StatusCode::BAD_REQUEST,
                AppError::GQLError(_) | AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let msg = err.to_string();
            let body = match req.format {
                OutputFormat::Text => QueryResultResponse::TextError(status, msg),
                OutputFormat::Json => QueryResultResponse::JsonError(
                    status,
                    Json(QueryError {
                        status: "error",
                        message: msg,
                    }),
                ),
            };
            (status, body).into_response()
        }
    }
}

async fn execute_query(query_str: &str, service: &QueryService) -> Result<Table, AppError> {
    let parsed_query = parse(query_str)?;
    let plan = QueryPlan::from(&parsed_query);
    let data = fetch_all_query_metrics(&plan).await?;
    let table = service.compute_table(&parsed_query, data).await?;

    Ok(table)
}

fn parse(query: &str) -> Result<Query, AppError> {
    let mut parsed = QueryParser::parse(Rule::query, query)?;
    let next = parsed.next().ok_or(ParseError("empty query".to_string()))?;
    parse_query(next)
}
