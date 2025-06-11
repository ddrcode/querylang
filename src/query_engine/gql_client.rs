use futures::future::try_join_all;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use std::{collections::HashMap, time::SystemTime};

use crate::{
    config,
    domain::{Metric, MetricData, SymbolData},
    error::AppError::{self, GQLError},
};

use super::{GetMetrics, QueryPlan, build_query_vars, get_metrics};

/// Asynchronously fetches metric data for all targets in the provided query plan.
/// Internally executes multiple GraphQL queries in parallel: one per target,
/// and returns a map from symbol name to metric data.
pub async fn fetch_all_query_metrics(plan: &QueryPlan) -> Result<SymbolData, AppError> {
    let to = SystemTime::now();
    let from = to.checked_sub(plan.range()).unwrap();
    let client = Client::new();

    let futures = plan.targets().map(|target| {
        let client = &client;
        let vars = build_query_vars(target, from, to, plan.step());
        async move { fetch_metrics(client, vars).await }
    });

    let data = try_join_all(futures).await?;
    Ok(data.into_iter().collect())
}

/// Sends a single GraphQL request for the given target variables and returns parsed metric data.
/// The result is a tuple of the queried symbol and a map of metrics to time series values.
pub async fn fetch_metrics(
    client: &Client,
    vars: get_metrics::Variables,
) -> Result<(String, MetricData), AppError> {
    let symbol = vars.symbol.clone();
    let request_body = GetMetrics::build_query(vars);

    // Optional debug log of the full request payload.
    if let Ok(json) = serde_json::to_string_pretty(&request_body) {
        tracing::debug!("GraphQL Query payload: {json}");
    }

    let res = client
        .post(config::GRAPHQL_SERVER)
        .json(&request_body)
        .send()
        .await?
        .json::<graphql_client::Response<get_metrics::ResponseData>>()
        .await?;

    let data = res.data.ok_or(GQLError("No data".to_string()))?;
    let metric_data = transform_response(data)?;

    Ok((symbol, metric_data))
}

/// Converts raw GraphQL response data into a map of metrics to float time series.
/// Expects the response to be grouped by timestamp, and flattens it into metric-centric form.
fn transform_response(data: get_metrics::ResponseData) -> Result<MetricData, AppError> {
    let mut result: MetricData = HashMap::new();

    for record in data.get_metrics {
        for value in record.values {
            let metric = Metric::try_from(value.metric.as_str())?;
            result.entry(metric).or_default().push(value.value as f32);
        }
    }

    Ok(result)
}
