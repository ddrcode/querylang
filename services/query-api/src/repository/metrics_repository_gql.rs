use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::future::try_join_all;
use graphql_client::GraphQLQuery;
use query_parser::Metric;
use reqwest;
use std::{collections::HashMap, time::Duration};

use crate::{
    domain::{MetricData, SymbolData},
    error::AppError::{self, GQLError},
    repository::MetricsRepository,
    shared::{DateRange, QueryPlan, TargetMetrics},
};

/// Represents the GraphQL query defined in `get_metrics.graphql`.
/// This struct is used by the `graphql_client` module to generate the request and response types.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/repository/schema.graphql",
    query_path = "src/repository/get_metrics.graphql",
    response_derives = "Debug"
)]
pub struct GetMetrics;

pub struct MetricsRepositoryGql {
    client: reqwest::Client,
    graphql_endpoint: String
}

impl MetricsRepositoryGql {
    pub fn new(graphql_endpoint: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            graphql_endpoint: graphql_endpoint.into()
        }
    }

    /// Constructs GraphQL query variables for a given data target and time window.
    /// The variables are required to intiialize GraphQL client.
    fn build_query_vars(
        &self,
        target: &TargetMetrics,
        range: &DateRange,
        step: Duration,
    ) -> get_metrics::Variables {
        let from_utc: DateTime<Utc> = range.from().into();
        let to_utc: DateTime<Utc> = range.to().into();

        get_metrics::Variables {
            symbol: target.symbol().into(),
            metrics: target.metrics().map(|m| m.to_string()).collect(),
            from: from_utc.to_rfc3339(),
            to: to_utc.to_rfc3339(),
            step: (step.as_secs() / 3600).to_string(),
        }
    }

    /// Sends a single GraphQL request for the given target variables and returns parsed metric data.
    /// The result is a tuple of the queried symbol and a map of metrics to time series values.
    pub async fn fetch_symbol_metrics(
        &self,
        vars: get_metrics::Variables,
    ) -> Result<MetricData, AppError> {
        let request_body = GetMetrics::build_query(vars);

        // Optional debug log of the full request payload.
        if let Ok(json) = serde_json::to_string_pretty(&request_body) {
            tracing::debug!("GraphQL Query payload: {json}");
        }

        let res = self
            .client
            .post(self.graphql_endpoint.clone())
            .json(&request_body)
            .send()
            .await?
            .json::<graphql_client::Response<get_metrics::ResponseData>>()
            .await?;

        let data = res.data.ok_or(GQLError("No data".to_string()))?;
        let metric_data = self.transform_response(data)?;

        Ok(metric_data)
    }

    /// Converts raw GraphQL response data into a map of metrics to float time series.
    /// Expects the response to be grouped by timestamp, and flattens it into metric-centric form.
    fn transform_response(&self, data: get_metrics::ResponseData) -> Result<MetricData, AppError> {
        let mut result: MetricData = HashMap::new();

        for record in data.get_metrics {
            for value in record.values {
                let metric = Metric::try_from(value.metric.as_str())?;
                result.entry(metric).or_default().push(value.value as f32);
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl MetricsRepository for MetricsRepositoryGql {
    async fn get_metrics_for_symbol(
        &self,
        metrics: &TargetMetrics,
        date_range: &DateRange,
        step: Duration,
    ) -> Result<MetricData, AppError> {
        let vars = self.build_query_vars(metrics, date_range, step);
        self.fetch_symbol_metrics(vars).await
    }

    async fn get_metrics_for_query_plan(&self, plan: &QueryPlan) -> Result<SymbolData, AppError> {
        let range = DateRange::from_now(plan.range());

        let futures = plan.targets().map(|target| {
            let vars = self.build_query_vars(target, &range, plan.step());
            async move { self.fetch_symbol_metrics(vars).await }
        });

        let data = try_join_all(futures).await?;
        Ok(plan
            .targets()
            .map(|t| t.symbol().to_string())
            .zip(data.into_iter())
            .collect())
    }
}
