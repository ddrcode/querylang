use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

pub async fn root_handler() -> impl IntoResponse {
    Json(Message {
        message: "OK".to_string(),
    })
}
