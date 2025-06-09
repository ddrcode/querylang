use crate::{parser::Expr, query_engine::SymbolData};

pub fn create_column(expr: &Expr, data: &SymbolData, size: usize) -> Vec<f32> {
    match expr {
        Expr::Value(val) => std::iter::repeat(val.clone() as f32)
            .take(size)
            .collect::<Vec<_>>(),

        Expr::Data(sm) => data[sm.symbol()][&sm.metric()].clone(),

        Expr::Binary(left, op, right) => {
            let left = create_column(left, data, size);
            let right = create_column(right, data, size);
            let opfn = op.opfn();
            left.iter()
                .zip(right)
                .map(|(a, b)| opfn(a.clone(), b.clone()))
                .collect::<Vec<f32>>()
        }
    }
}
