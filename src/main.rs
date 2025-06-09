pub mod http_server;
pub mod error;
pub mod parser;

use http_server::start_server;
use error::AppError;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    start_server("0.0.0.0:3000").await;
    Ok(())
}
