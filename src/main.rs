pub mod http_server;
pub mod error;
pub mod adapter;
pub mod query_engine;
pub mod data;
pub mod config;
pub mod domain;

use http_server::start_server;
use error::AppError;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    start_server(config::QUERY_SERVER).await;

    Ok(())
}
