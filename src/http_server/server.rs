use super::create_router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
// use std::time::Duration;
// use tower::timeout::TimeoutLayer;

pub async fn start_server(path: &str) {
    let listener = TcpListener::bind(path).await.unwrap();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        // .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .into_inner();


    let app = create_router();

    axum::serve(listener, app.layer(middleware_stack)).await.unwrap();
}
