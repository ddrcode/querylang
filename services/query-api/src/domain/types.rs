use std::collections::HashMap;
use query_parser::Metric;

pub type MetricData = HashMap<Metric, Vec<f32>>;
pub type SymbolData = HashMap<String, MetricData>;
