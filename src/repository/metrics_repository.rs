use std::time::Duration;

use crate::{
    domain::{MetricData, SymbolData},
    error::AppError,
    shared::{DateRange, QueryPlan, TargetMetrics},
};

#[async_trait::async_trait]
pub trait MetricsRepository: Send + Sync {
    async fn get_metrics_for_symbol(
        &self,
        metrics: &TargetMetrics,
        date_range: &DateRange,
        step: Duration,
    ) -> Result<MetricData, AppError>;

    async fn get_metrics_for_query_plan(
        &self,
        query_plan: &QueryPlan,
    ) -> Result<SymbolData, AppError>;
}
