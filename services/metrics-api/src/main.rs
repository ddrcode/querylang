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
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

use crate::{
    api::{
        graphql::{build_schema, graphql_handler},
        root_handler,
    },
    repository::MetricsRepositoryMock,
    service::MetricsService,
};

#[tokio::main]
async fn main() {
    let mock_repo = MetricsRepositoryMock::new();
    let metrics_srv = MetricsService::new(Arc::new(mock_repo));
    let schema = build_schema(Arc::new(metrics_srv));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Running at http://{}/graphql", addr);

    serve(listener, app).await.unwrap();
}
