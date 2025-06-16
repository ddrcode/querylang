pub mod api;
pub mod domain;
pub mod error;
pub mod repository;
pub mod service;
pub mod shared;

use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{Extension, Router, routing::post};
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

    let config = load_config()?;

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

fn load_config() -> Result<Config, anyhow::Error> {
    let file: String = if fs::exists(Path::new("config/default.toml"))? {
        "config/default.toml".into()
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join("default.toml")
            .to_str()
            .expect("Path is not a valid UTF-8").into()
    };
    let content = fs::read_to_string(file)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
