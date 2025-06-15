use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct MetricValue {
    pub metric: String,
    pub value: f32,
}

