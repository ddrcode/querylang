use chrono::{DateTime, Utc};

use crate::{
    error::MetricsApiError, repository::MetricsRepository, shared::{MetricRecord, MetricValue}
};

pub struct MetricsRepositoryMock {}

impl MetricsRepositoryMock {
    pub fn new() -> Self {
        Self {}
    }

    fn generate_metric_val(&self, metric: &str) -> f32 {
        let base = 100.0 + rand::random::<f32>() * 50.0;
        match metric {
            "max" => base * 1.12,
            "min" => base * 0.75,
            "volume" => base * 15.0,
            _ => base,
        }
    }
}

#[async_trait::async_trait]
impl MetricsRepository for MetricsRepositoryMock {
    async fn get_metrics(
        &self,
        _symbol: String,
        metrics: Vec<String>,
        from: String,
        to: String,
        step: String,
    ) -> Result<Vec<MetricRecord>, MetricsApiError> {
        let step_hours: u64 = step.parse().unwrap_or(1);
        let from = DateTime::parse_from_rfc3339(&from)
            .unwrap()
            .with_timezone(&Utc);
        let to = DateTime::parse_from_rfc3339(&to)
            .unwrap()
            .with_timezone(&Utc);

        let mut records = vec![];
        let mut current = from;

        while current < to {
            let timestamp = current.to_rfc3339();
            let values = metrics
                .iter()
                .map(|metric| MetricValue {
                    metric: metric.clone(),
                    value: self.generate_metric_val(metric),
                })
                .collect();

            records.push(MetricRecord { timestamp, values });
            current += chrono::Duration::hours(step_hours as i64);
        }

        Ok(records)
    }
}
