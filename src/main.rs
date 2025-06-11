pub mod adapter;
pub mod config;
pub mod domain;
pub mod error;
pub mod api;
pub mod query_engine;
pub mod service;
pub mod shared;

use axum::{routing::post, Extension, Router};
use error::AppError;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::service::QueryService;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let query_srv = QueryService::new();

    let app = Router::new()
        .route("/", axum::routing::get(api::root_handler))
        .route("/query", post(api::query_handler))
        .layer(Extension(query_srv));

    let listener = TcpListener::bind(config::QUERY_SERVER).await.unwrap();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    axum::serve(listener, app.layer(middleware_stack)).await.unwrap();

    Ok(())
}
