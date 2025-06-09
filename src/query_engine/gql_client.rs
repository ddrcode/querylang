use get_metrics::ResponseData;
use graphql_client::GraphQLQuery;
use reqwest::Client;

use crate::error::AppError::{self, GQLError};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/query_engine/schema.graphql",
    query_path = "src/query_engine/get_metrics.graphql",
    response_derives = "Debug"
)]
pub struct GetMetrics;

pub struct GraphQLClient {
    server_url: String,
}

impl GraphQLClient {
    pub fn new(server_url: &str) -> Self {
        Self {
            server_url: server_url.to_string(),
        }
    }

    pub async fn fetch_metrics(&self) -> Result<String, AppError> {
        let request_body = GetMetrics::build_query(get_metrics::Variables {
            symbol: "AAPL".into(),
            metrics: vec!["open".into(), "volume".into()],
            from: "2023-06-01T00:00:00".into(),
            to: "2023-06-02T00:00:00".into(),
            step: "1h".into(),
        });

        if let Ok(json) = serde_json::to_string_pretty(&request_body.variables) {
            tracing::debug!("GraphQL Query variables: {json}");
        }

        let client = Client::new();
        let res = client
            .post(&self.server_url)
            .json(&request_body)
            .send()
            .await?
            .text()
            // .json::<graphql_client::Response<get_metrics::ResponseData>>()
            .await?;

        Ok(res)

        // res.data.ok_or(GQLError("Empty response".to_string()))
    }
}
