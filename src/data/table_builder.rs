use std::sync::Arc;

use crate::{error::AppError, parser::Query, query_engine::SymbolData};
use super::{Table, compute_all_columns};

/// Converts data from grapql server into an output table
/// First it computes all columns (applying query expressions)
/// Then it trposes the results. It also adds headers row and time step column
pub async fn compute_table(query: &Query, data: SymbolData) -> Result<Table, AppError> {
    let data = Arc::new(data);

    let mut columns: Vec<Vec<f32>> = vec![timestamps_column(&query)];
    columns.extend(compute_all_columns(query.expressions(), data, query.rows_count()).await?);

    let rows = transpose(columns);

    let mut headers = vec!["timestamp".to_string()];
    headers.extend(query.expressions().iter().map(|expr| expr.to_string()));

    Ok(Table::new(headers, rows))
}

/// Converts vector of columns into vector of rows
fn transpose(columns: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let row_count = columns[1].len();
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
fn timestamps_column(query: &Query) -> Vec<f32> {
    std::iter::successors(Some(0f32), move |prev| {
        Some(prev + (query.step().value() as f32))
    })
    .take(query.rows_count())
    .collect()
}
