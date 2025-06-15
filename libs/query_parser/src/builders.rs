use super::error::ParseError;
use pest::iterators::Pair;

use super::{
    model::{Expr, Metric, Operator, Query, SymbolMetric, TimeSpec, TimeUnit},
    parser::Rule,
};

type ParseResult<T> = Result<T, ParseError>;

pub(crate) fn build_query(pair: Option<Pair<Rule>>) -> ParseResult<Query> {
    let pair = pair.ok_or(ParseError::MissingPair("query".into()))?;
    expect_rule(&pair, Rule::query)?;
    let mut pairs = pair.into_inner();

    let exprs = build_expr_list(pairs.next())?;
    let for_clause = build_for_clause(pairs.next())?;
    let step_clause = build_step_clause(pairs.next())?;

    Ok(Query::new(exprs, for_clause, step_clause))
}

pub(crate) fn build_expr_list(pair: Option<Pair<Rule>>) -> ParseResult<Vec<Expr>> {
    let pair = pair.ok_or(ParseError::MissingPair("expr_list".into()))?;
    pair.into_inner()
        .map(|expr| build_expr(Some(expr)))
        .collect()
}

pub(crate) fn build_expr(pair: Option<Pair<Rule>>) -> Result<Expr, ParseError> {
    let pair = pair.ok_or(ParseError::MissingPair("expr".into()))?;
    expect_rule(&pair, Rule::expr)?;

    let mut inner = pair.into_inner();
    let mut left = build_term(inner.next())?;

    while let Some(op_pair) = inner.next() {
        match op_pair.as_rule() {
            Rule::expr_op => {
                let right = build_term(inner.next())?;
                let op = Operator::try_from(op_pair.as_str())?;
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            other => {
                return Err(ParseError::InvalidRule(
                    "expr".into(),
                    other.to_string().into(),
                ));
            }
        }
    }

    Ok(left)
}

pub(crate) fn build_term(pair: Option<Pair<Rule>>) -> ParseResult<Expr> {
    let pair = pair.ok_or(ParseError::MissingPair("term".into()))?;

    let mut inner = pair.into_inner();
    let next_pair = inner.next();
    let mut left = build_factor(next_pair)?;

    while let Some(next_pair) = inner.next() {
        match next_pair.as_rule() {
            Rule::term_op => {
                let op = Operator::try_from(next_pair.as_str())?;
                let next_pair = inner.next();
                let right = build_factor(next_pair)?;
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            other => {
                return Err(ParseError::InvalidRule(
                    "term_op".into(),
                    other.to_string().into(),
                ));
            }
        }
    }

    Ok(left)
}

pub(crate) fn build_factor(pair: Option<Pair<Rule>>) -> ParseResult<Expr> {
    let pair = pair.ok_or(ParseError::MissingPair("factor".into()))?;
    expect_rule(&pair, Rule::factor)?;

    let mut inner = pair.into_inner();
    let pair = inner
        .next()
        .ok_or(ParseError::MissingPair("factor".into()))?;

    let val = match pair.as_rule() {
        Rule::data => {
            let mut inner = pair.into_inner();
            let symbol = build_symbol(inner.next())?;
            let metric = build_metric(inner.next())?;
            Expr::Data(SymbolMetric::new(&symbol, metric))
        }
        Rule::value => Expr::Value(build_value(Some(pair))?),
        Rule::expr => build_expr(Some(pair))?, // for grouped expressions: (a + b)
        other => {
            return Err(ParseError::InvalidRule(
                "data, value or expr".into(),
                other.to_string().into(),
            ));
        }
    };
    Ok(val)
}

pub(crate) fn build_step_clause(pair: Option<Pair<Rule>>) -> ParseResult<TimeSpec> {
    let pair = pair.ok_or(ParseError::MissingPair("step_clause".into()))?;
    expect_rule(&pair, Rule::step_clause)?;

    let mut inner = pair.into_inner();
    let value = build_value(inner.next())?;
    let unit = build_time_unit(inner.next())?;

    Ok(TimeSpec::new(value, unit))
}

pub(crate) fn build_for_clause(pair: Option<Pair<Rule>>) -> ParseResult<TimeSpec> {
    let pair = pair.ok_or(ParseError::MissingPair("for_clause".into()))?;
    expect_rule(&pair, Rule::for_clause)?;

    let mut inner = pair.into_inner();
    let value = build_value(inner.next())?;
    let unit = build_time_unit(inner.next())?;

    Ok(TimeSpec::new(value, unit))
}

pub(crate) fn build_symbol(pair: Option<Pair<Rule>>) -> ParseResult<String> {
    let val = pair.ok_or(ParseError::MissingPair("symbol".into()))?;
    expect_rule(&val, Rule::symbol)?;
    Ok(val.as_str().to_string())
}

pub(crate) fn build_metric(pair: Option<Pair<Rule>>) -> ParseResult<Metric> {
    let val = pair.ok_or(ParseError::MissingPair("metric".into()))?;
    expect_rule(&val, Rule::metric)?;
    Metric::try_from(val.as_str())
}

pub(crate) fn build_value(pair: Option<Pair<Rule>>) -> ParseResult<u32> {
    let val = pair.ok_or(ParseError::MissingPair("value".into()))?;
    expect_rule(&val, Rule::value)?;
    let valstr = val.as_str().to_string();

    valstr
        .parse()
        .map_err(|_| ParseError::InvalidValue(valstr.into(), "value".into()))
}

pub(crate) fn build_time_unit(pair: Option<Pair<Rule>>) -> ParseResult<TimeUnit> {
    let val = pair.ok_or(ParseError::MissingPair("timeunit".into()))?;
    expect_rule(&val, Rule::time_unit)?;
    TimeUnit::try_from(val.as_str())
}

fn expect_rule(pair: &Pair<Rule>, expected: Rule) -> ParseResult<()> {
    let rule = pair.as_rule();
    if rule != expected {
        return Err(ParseError::InvalidRule(
            expected.to_string().into(),
            rule.to_string().into(),
        ));
    }
    Ok(())
}
