use std::sync::Arc;

use async_graphql::Object;

use crate::{error::MetricsApiError, service::MetricsService, shared::MetricRecord};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_metrics(
        &self,
        ctx: &async_graphql::Context<'_>,
        symbol: String,
        metrics: Vec<String>,
        from: String,
        to: String,
        step: String,
    ) -> Result<Vec<MetricRecord>, MetricsApiError> {
        let service = ctx.data::<Arc<MetricsService>>().unwrap();
        service.get_metrics_for_symbol(symbol, metrics, from, to, step).await
    }
}

