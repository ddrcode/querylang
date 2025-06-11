use axum::{Json, response::IntoResponse};
use crate::shared::StatusMsg;

pub async fn root_handler() -> impl IntoResponse {
    Json(StatusMsg::from_str("ok", "System is running"))
}
