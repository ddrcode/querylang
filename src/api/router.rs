use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use serde::Serialize;

use crate::service::QueryService;

use super::handle_query;

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn root() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Message {
        message: "OK".to_string(),
    }))
}

pub fn create_router() -> Router {

    let query_srv = QueryService::new();

    Router::new()
        .route("/", get(root))
        .route("/query", post(handle_query))
        .layer(Extension(query_srv))
}
