use crate::{error::MetricsApiError, repository::MetricsRepository, shared::MetricRecord};
use std::sync::Arc;

pub struct MetricsService {
    metrics_repo: Arc<dyn MetricsRepository>,
}

impl MetricsService {
    pub fn new(metrics_repo: Arc<dyn MetricsRepository>) -> Self {
        Self { metrics_repo }
    }

    pub async fn get_metrics_for_symbol(
        &self,
        symbol: String,
        metrics: Vec<String>,
        from: String,
        to: String,
        step: String,
    ) -> Result<Vec<MetricRecord>, MetricsApiError> {
        self.metrics_repo
            .get_metrics(symbol, metrics, from, to, step)
            .await
    }
}
