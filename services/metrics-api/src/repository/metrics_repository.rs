use crate::{error::MetricsApiError, shared::MetricRecord};

#[async_trait::async_trait]
pub trait MetricsRepository: Send + Sync {
    async fn get_metrics(
        &self,
        symbol: String,
        metrics: Vec<String>,
        from: String,
        to: String,
        step: String,
    ) -> Result<Vec<MetricRecord>, MetricsApiError>;
}
