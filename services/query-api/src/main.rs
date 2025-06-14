pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repository;
pub mod service;
pub mod shared;

use std::sync::Arc;

use axum::{Extension, Router, routing::post};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::{repository::MetricsRepositoryGql, service::QueryService};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let metrics_repo = MetricsRepositoryGql::new();
    let query_srv = QueryService::new(Arc::new(metrics_repo));

    let app = Router::new()
        .route("/", axum::routing::get(api::root_handler))
        .route("/query", post(api::query_handler))
        .layer(Extension(query_srv));

    let listener = TcpListener::bind(config::QUERY_SERVER).await?;

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    axum::serve(listener, app.layer(middleware_stack)).await?;

    Ok(())
}
