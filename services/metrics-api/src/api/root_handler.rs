use async_graphql::http::GraphQLPlaygroundConfig;
use axum::response::{Html, IntoResponse};

pub async fn root_handler() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
