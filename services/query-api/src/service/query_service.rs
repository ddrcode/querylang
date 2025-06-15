use std::sync::Arc;

use futures::future::try_join_all;
use query_parser::{Expr, Query};
use tokio::task;

use crate::{
    domain::{SymbolData, Table},
    error::AppError,
    repository::MetricsRepository,
    shared::QueryPlan,
};

#[derive(Clone)]
pub struct QueryService {
    metrics_repo: Arc<dyn MetricsRepository>,
}

impl QueryService {
    pub fn new(metrics_repo: Arc<dyn MetricsRepository>) -> Self {
        Self { metrics_repo }
    }

    pub async fn run_query(&self, query: &Query) -> Result<Table, AppError> {
        let plan = QueryPlan::from(query);
        let data = self.metrics_repo.get_metrics_for_query_plan(&plan).await?;
        self.compute_table(query, data).await
    }

    async fn compute_table(&self, query: &Query, data: SymbolData) -> Result<Table, AppError> {
        let data = Arc::new(data);

        let mut columns: Vec<Vec<f32>> = vec![self.timestamps_column(&query)];
        columns.extend(
            self.compute_all_columns(query.expressions(), data, query.rows_count())
                .await?,
        );

        let rows = self.transpose(columns);

        let mut headers = vec!["time step".to_string()];
        headers.extend(query.expressions().iter().map(|expr| expr.to_string()));

        Ok(Table::new(headers, rows))
    }

    async fn compute_all_columns(
        &self,
        exprs: &[Expr],
        symbol: Arc<SymbolData>,
        size: usize,
    ) -> Result<Vec<Vec<f32>>, AppError> {
        let exprs = exprs.to_vec();

        let tasks = exprs.into_iter().map(|expr| {
            let data = Arc::clone(&symbol);
            task::spawn_blocking(move || create_column(&expr, &data, size))
        });

        let results = try_join_all(tasks)
            .await
            .map_err(|e| AppError::DataError(format!("Error while computing columns: {e}")))?;

        results.into_iter().collect()
    }

    /// Converts vector of columns into vector of rows
    fn transpose(&self, columns: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let row_count = columns[0].len();
        let col_count = columns.len();

        let mut rows: Vec<Vec<f32>> = (0..row_count)
            .map(|_| Vec::with_capacity(col_count))
            .collect();

        for col in columns {
            for (i, value) in col.into_iter().enumerate() {
                rows[i].push(value);
            }
        }

        rows
    }

    /// Produces vector of timestamps (or - precisely - time steps)
    fn timestamps_column(&self, query: &Query) -> Vec<f32> {
        std::iter::successors(Some(0f32), move |prev| {
            Some(prev + (query.step().value() as f32))
        })
        .take(query.rows_count())
        .collect()
    }
}

fn create_column(expr: &Expr, data: &SymbolData, size: usize) -> Result<Vec<f32>, AppError> {
    let col = match expr {
        Expr::Value(val) => std::iter::repeat(val.clone() as f32)
            .take(size)
            .collect::<Vec<_>>(),

        Expr::Data(sm) => data[sm.symbol()][&sm.metric()].clone(),

        Expr::Binary(left, op, right) => {
            let left = create_column(left, data, size)?;
            let right = create_column(right, data, size)?;
            let opfn = op.opfn();
            left.iter()
                .zip(right)
                .map(|(a, b)| opfn(a.clone(), b.clone()))
                .collect::<Vec<f32>>()
        }
    };
    Ok(col)
}
