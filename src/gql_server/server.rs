use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, http::GraphQLPlaygroundConfig,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::{get, post},
    serve,
};
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(SimpleObject)]
struct MetricValue {
    metric: String,
    value: f32,
}

#[derive(SimpleObject)]
struct MetricRecord {
    timestamp: String,
    values: Vec<MetricValue>,
}

#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_metrics(
        &self,
        symbol: String,
        metrics: Vec<String>,
        from: String,
        to: String,
        step: String,
    ) -> Vec<MetricRecord> {
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
                    value: generate_metric_val(metric)
                })
                .collect();

            records.push(MetricRecord { timestamp, values });
            current += chrono::Duration::hours(step_hours as i64);
        }

        records
    }
}

fn generate_metric_val(metric: &str) -> f32 {
    let base = 100.0 + rand::random::<f32>() * 50.0;
    match metric {
        "max" => base * 1.12,
        "min" => base * 0.75,
        "volume" => base * 15.0,
        _ => base
    }
}

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(State(schema): State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Running at http://{}/graphql", addr);

    serve(listener, app).await.unwrap();
}
