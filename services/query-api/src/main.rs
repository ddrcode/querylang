pub mod api;
pub mod domain;
pub mod error;
pub mod repository;
pub mod service;
pub mod shared;

use std::sync::Arc;

use axum::{Extension, Router, routing::post};
use common::utils::load_config;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::{repository::MetricsRepositoryGql, service::QueryService, shared::Config};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = load_config::<Config>(env!("CARGO_MANIFEST_DIR"))?;

    let metrics_repo = MetricsRepositoryGql::new(&config.graphql_server);
    let query_srv = QueryService::new(Arc::new(metrics_repo));

    let app = Router::new()
        .route("/", axum::routing::get(api::root_handler))
        .route("/query", post(api::query_handler))
        .layer(Extension(query_srv));

    let listener = TcpListener::bind(config.query_server).await?;

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .into_inner();

    axum::serve(listener, app.layer(middleware_stack)).await?;

    Ok(())
}
