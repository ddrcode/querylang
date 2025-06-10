use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use graphql_client::GraphQLQuery;

use super::DataTarget;

/// Represents the GraphQL query defined in `get_metrics.graphql`.
/// This struct is used by the `graphql_client` module to generate the request and response types.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/query_engine/schema.graphql",
    query_path = "src/query_engine/get_metrics.graphql",
    response_derives = "Debug"
)]
pub struct GetMetrics;


/// Constructs GraphQL query variables for a given data target and time window.
/// The variables are required to intiialize GraphQL client.
pub fn build_query_vars(
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
