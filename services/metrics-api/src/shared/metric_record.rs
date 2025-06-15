use async_graphql::SimpleObject;
use super::MetricValue;

#[derive(SimpleObject)]
pub struct MetricRecord {
    pub timestamp: String,
    pub values: Vec<MetricValue>,
}


