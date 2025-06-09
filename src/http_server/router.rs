use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[derive(Serialize)]
struct Message {
    message: String
}

async fn root() -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(Message {
        message: "OK".to_string()
    }))
}

async fn run_query(Json(query): Json<Query>) -> String {
    format!("You sent: {}", query.query)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/query", post(run_query))
}
