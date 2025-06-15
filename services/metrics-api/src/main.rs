pub mod api;
pub mod error;
pub mod repository;
pub mod service;
pub mod shared;

use axum::{
    Router,
    routing::{get, post},
    serve,
};
// use common::utils::load_config;
use std::{fs, sync::Arc};
use tokio::net::TcpListener;
use toml;

use crate::{
    api::{
        graphql::{build_schema, graphql_handler},
        root_handler,
    },
    repository::MetricsRepositoryMock,
    service::MetricsService,
    shared::Config,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let config = load_config<Config>(env!("CARGO_MANIFEST_DIR"))?;
    let config = load_config()?;

    let mock_repo = MetricsRepositoryMock::new();
    let metrics_srv = MetricsService::new(Arc::new(mock_repo));
    let schema = build_schema(Arc::new(metrics_srv));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let listener = TcpListener::bind(config.metrics_server).await?;

    serve(listener, app).await.unwrap();

    Ok(())
}

fn load_config() -> Result<Config, anyhow::Error> {
    let content = fs::read_to_string("config/default.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
