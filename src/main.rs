pub mod adapter;
pub mod config;
pub mod data;
pub mod domain;
pub mod error;
pub mod http_server;
pub mod query_engine;
pub mod service;
pub mod shared;

use error::AppError;
use http_server::start_server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    start_server(config::QUERY_SERVER).await;

    Ok(())
}
