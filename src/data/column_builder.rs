use futures::future::try_join_all;
use std::sync::Arc;
use tokio::task;

use crate::{error::AppError, parser::Expr, query_engine::SymbolData};

pub fn create_column(expr: &Expr, data: &SymbolData, size: usize) -> Result<Vec<f32>, AppError> {
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

pub async fn compute_all_columns(
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
