use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Utc};
use futures::future::try_join_all;
use graphql_client::GraphQLQuery;
use reqwest::Client;

use crate::{
    config,
    error::AppError::{self, GQLError},
    parser::Metric,
};

use super::{DataTarget, QueryPlan};

type MetricData = HashMap<Metric, Vec<f32>>;
type SymbolData = HashMap<String, MetricData>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/query_engine/schema.graphql",
    query_path = "src/query_engine/get_metrics.graphql",
    response_derives = "Debug"
)]
pub struct GetMetrics;

pub fn build_vars(
    target: &DataTarget,
    from: SystemTime,
    to: SystemTime,
    step: Duration,
) -> get_metrics::Variables {
    let from_utc: DateTime<Utc> = from.into();
    let to_utc: DateTime<Utc> = to.into();

    get_metrics::Variables {
        symbol: target.symbol().into(),
        metrics: target.metrics().map(|m| m.to_string()).collect(),
        from: from_utc.to_rfc3339(),
        to: to_utc.to_rfc3339(),
        step: (step.as_secs() / 3600).to_string(),
    }
}

pub async fn fetch_all_query_metrics(
    plan: &QueryPlan,
) -> Result<HashMap<String, MetricData>, AppError> {
    let to = SystemTime::now();
    let from = to.checked_sub(plan.range()).unwrap();
    let client = Client::new();

    let futures = plan.targets().map(|target| {
        let client = &client;
        let vars = build_vars(target, from, to, plan.step());
        async move { fetch_metrics(client, vars).await }
    });

    let data = try_join_all(futures).await?;
    let result = data.into_iter().collect::<HashMap<_, _>>();

    Ok(result)
}

pub async fn fetch_metrics(
    client: &Client,
    vars: get_metrics::Variables,
) -> Result<(String, MetricData), AppError> {
    let symbol = vars.symbol.clone();
    let request_body = GetMetrics::build_query(vars);

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
    let map = transform_response(data)?;

    Ok((symbol, map))
}

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
