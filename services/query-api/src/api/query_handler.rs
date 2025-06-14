use axum::{
    Extension, Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    domain::Table,
    error::AppError,
    service::QueryService,
};
use query_parser::{self, Query, QueryParser, Rule, parse_query};
use common::shared::StatusMsg;

#[derive(Deserialize)]
pub struct QueryReq {
    query: String,
    #[serde(default)]
    format: OutputFormat,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Text,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Text
    }
}

pub enum QueryResultResponse {
    OkJson(Json<Table>),
    OkText(String),
    ErrorJson(StatusCode, Json<StatusMsg>),
    ErrorText(StatusCode, String),
}

impl IntoResponse for QueryResultResponse {
    fn into_response(self) -> Response {
        use QueryResultResponse::*;
        match self {
            OkJson(table) => table.into_response(),
            OkText(txt) => ([(header::CONTENT_TYPE, "text/plain")], txt).into_response(),
            ErrorJson(code, err) => (code, err).into_response(),
            ErrorText(code, msg) => {
                (code, [(header::CONTENT_TYPE, "text/plain")], msg).into_response()
            }
        }
    }
}

pub async fn query_handler(
    Extension(service): Extension<QueryService>,
    Json(req): Json<QueryReq>,
) -> impl IntoResponse {
    use QueryResultResponse::*;
    let result = execute_query(&req.query, &service).await;

    match (result, req.format) {
        (Ok(table), OutputFormat::Text) => OkText(table.to_string()).into_response(),

        (Ok(table), OutputFormat::Json) => OkJson(Json(table)).into_response(),

        (Err(err), format) => {
            let status = match err {
                AppError::ParseError(_) => StatusCode::BAD_REQUEST,
                AppError::GQLError(_) | AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let message = err.to_string();
            let body = match format {
                OutputFormat::Text => ErrorText(status, message),
                OutputFormat::Json => ErrorJson(status, Json(StatusMsg::error(message))),
            };
            (status, body).into_response()
        }
    }
}

async fn execute_query(query_str: &str, service: &QueryService) -> Result<Table, AppError> {
    let parsed_query = parse(query_str)?;
    let table = service.run_query(&parsed_query).await?;
    Ok(table)
}

fn parse(query: &str) -> Result<Query, AppError> {
    let mut parsed = QueryParser::parse(Rule::query, query)?;
    let next = parsed.next().ok_or(query_p("empty query".to_string()))?;
    Ok(parse_query(next)?)
}
