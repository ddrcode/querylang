use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

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
    Router::new()
        .route("/", get(root))
        .route("/query", post(handle_query))
}
