use std::sync::Arc;

use crate::{error::AppError, parser::Expr, query_engine::SymbolData};

use super::{compute_all_columns, Table};

pub async fn compute_table(
    exprs: &[Expr],
    data: SymbolData,
    size: usize,
) -> Result<Table, AppError> {
    let data = Arc::new(data);
    let columns = compute_all_columns(exprs, data, size).await?;
    let rows = transpose(columns);
    let headers = exprs.iter().map(|expr| expr.to_string()).collect();

    Ok(Table::new(headers, rows))
}

fn transpose(columns: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let row_count = columns[0].len();
    let col_count = columns.len();

    let mut rows = vec![Vec::with_capacity(col_count); row_count];

    for col in columns {
        for (i, value) in col.into_iter().enumerate() {
            rows[i].push(value);
        }
    }

    rows
}
