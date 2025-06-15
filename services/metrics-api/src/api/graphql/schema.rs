use std::sync::Arc;

use crate::service::MetricsService;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use super::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(service: Arc<MetricsService>) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(service.clone())
        .finish()
}
