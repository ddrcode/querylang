use super::error::ParseError;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::model::{Expr, Metric, Operator, Query, SymbolMetric, TimeSpec, TimeUnit};

type ParseResult<T> = Result<T, ParseError>;

#[derive(Parser)]
#[grammar = "src/query.pest"]
pub struct QueryParser;

pub fn parse_query(pair: Pair<Rule>) -> ParseResult<Query> {
    assert_eq!(pair.as_rule(), Rule::query);
    let mut pairs = pair.into_inner();

    let exprs = parse_expr_list(pairs.next())?;
    let for_clause = parse_for_clause(pairs.next())?;
    let step_clause = parse_step_clause(
        pairs
            .next()
    )?;

    Ok(Query::new(exprs, for_clause, step_clause))
}

fn parse_expr_list(pair: Option<Pair<Rule>>) -> ParseResult<Vec<Expr>> {
    let pair = pair.ok_or(ParseError::MissingPair("expr_list"))?;
    pair.into_inner().map(parse_expr).collect()
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    expect_rule(&pair, Rule::expr)?;

    let mut inner = pair.into_inner();
    let mut left = parse_term(inner.next())?;

    while let Some(op_pair) = inner.next() {
        match op_pair.as_rule() {
            Rule::expr_op => {
                let right = parse_term(inner.next())?;
                let op = Operator::try_from(op_pair.as_str())?;
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            other => return Err(ParseError::InvalidRule("expr".into(), format!("{other:?}"))),
        }
    }

    Ok(left)
}

fn parse_term(pair: Option<Pair<Rule>>) -> ParseResult<Expr> {
    let pair = pair.ok_or(ParseError::MissingPair("term"))?;

    let mut inner = pair.into_inner();
    let next_pair = inner.next();
    let mut left = parse_factor(next_pair)?;

    while let Some(next_pair) = inner.next() {
        match next_pair.as_rule() {
            Rule::term_op => {
                let op = Operator::try_from(next_pair.as_str())?;
                let next_pair = inner.next();
                let right = parse_factor(next_pair)?;
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            other => {
                return Err(ParseError::InvalidRule(
                    "term_op".into(),
                    format!("{other:?}"),
                ));
            }
        }
    }

    Ok(left)
}

fn parse_factor(pair: Option<Pair<Rule>>) -> ParseResult<Expr> {
    let pair = pair.ok_or(ParseError::MissingPair("factor"))?;
    expect_rule(&pair, Rule::factor)?;

    let mut inner = pair.into_inner();
    let pair = inner.next().ok_or(ParseError::MissingPair("factor"))?;

    let val = match pair.as_rule() {
        Rule::data => {
            let mut inner = pair.into_inner();
            let symbol = parse_symbol(inner.next())?;
            let metric = parse_metric(inner.next())?;
            Expr::Data(SymbolMetric::new(&symbol, metric))
        }
        Rule::value => Expr::Value(parse_value(Some(pair))?),
        Rule::expr => parse_expr(pair)?, // for grouped expressions: (a + b)
        other => {
            return Err(ParseError::InvalidRule(
                "data, value or expr".into(),
                format!("{other:?}"),
            ));
        }
    };
    Ok(val)
}

fn parse_step_clause(pair: Option<Pair<Rule>>) -> ParseResult<TimeSpec> {
    let pair = pair.ok_or(ParseError::MissingPair("step_clause"))?;
    expect_rule(&pair, Rule::step_clause)?;

    let mut inner = pair.into_inner();
    let value = parse_value(inner.next())?;
    let unit = parse_time_unit(inner.next())?;

    Ok(TimeSpec::new(value, unit))
}

fn parse_for_clause(pair: Option<Pair<Rule>>) -> ParseResult<TimeSpec> {
    let pair = pair.ok_or(ParseError::MissingPair("for_clause"))?;
    expect_rule(&pair, Rule::for_clause)?;

    let mut inner = pair.into_inner();
    let value = parse_value(inner.next())?;
    let unit = parse_time_unit(inner.next())?;

    Ok(TimeSpec::new(value, unit))
}

fn parse_symbol(pair: Option<Pair<Rule>>) -> ParseResult<String> {
    let val = pair.ok_or(ParseError::MissingPair("symbol"))?;
    expect_rule(&val, Rule::symbol)?;
    Ok(val.as_str().to_string())
}

fn parse_metric(pair: Option<Pair<Rule>>) -> ParseResult<Metric> {
    let val = pair.ok_or(ParseError::MissingPair("metric"))?;
    expect_rule(&val, Rule::metric)?;
    Metric::try_from(val.as_str())
}

fn parse_value(pair: Option<Pair<Rule>>) -> ParseResult<u32> {
    let val = pair.ok_or(ParseError::MissingPair("value"))?;
    expect_rule(&val, Rule::value)?;
    let valstr = val.as_str().to_string();

    valstr
        .parse()
        .map_err(|_| ParseError::InvalidValue(valstr, "value".into()))
}

fn parse_time_unit(pair: Option<Pair<Rule>>) -> ParseResult<TimeUnit> {
    let val = pair.ok_or(ParseError::MissingPair("timeunit"))?;
    expect_rule(&val, Rule::time_unit)?;
    TimeUnit::try_from(val.as_str())
}

fn expect_rule(pair: &Pair<Rule>, expected: Rule) -> ParseResult<()> {
    let rule = pair.as_rule();
    if rule != expected {
        return Err(ParseError::InvalidRule(
            format!("{expected:?}"),
            format!("{rule:?}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    fn debug_pair(pair: &Pair<Rule>, indent: usize) {
        println!(
            "{:indent$}- {:?}: {:?}",
            "",
            pair.as_rule(),
            pair.as_str(),
            indent = indent
        );
        for inner in pair.clone().into_inner() {
            debug_pair(&inner, indent + 2);
        }
    }

    fn parse(rule: Rule, input: &str) -> Pair<Rule> {
        let parsed = QueryParser::parse(rule, input)
            .expect("parse failed")
            .next()
            .unwrap();
        debug_pair(&parsed, 0);
        parsed
    }

    #[test]
    fn test_value_expr_parse() {
        let input = r"32";
        let parsed = parse(Rule::expr, input);
        let expr = parse_expr(parsed).unwrap();
        assert_eq!(Expr::Value(32), expr);
    }

    #[test]
    fn test_symbol_expr_parse() {
        let input = r"AAPL.max";
        let parsed = parse(Rule::expr, input);
        let expr = parse_expr(parsed).unwrap();
        assert_eq!(Expr::Data(SymbolMetric::new("AAPL", Metric::Max)), expr);
    }

    #[test]
    fn test_full_expr_parse() {
        let input = r"AAPL.volume / 1000";
        let parsed = parse(Rule::expr, input);
        let expr = parse_expr(parsed).unwrap();
        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Data(SymbolMetric::new("AAPL", Metric::Volume))),
                Operator::Div,
                Box::new(Expr::Value(1000))
            ),
            expr
        );
    }

    #[test]
    fn test_for_clause() {
        let input = r"FOR LAST 10 days";
        let parsed = parse(Rule::for_clause, input);
        let time = parse_for_clause(Some(parsed)).unwrap();
        assert_eq!(10, time.value());
        assert_eq!(TimeUnit::Day, time.unit());
    }

    #[test]
    fn test_step_clause() {
        let input = r"STEP 1 day";
        let parsed = parse(Rule::step_clause, input);
        debug_pair(&parsed, 0);
        let step = parse_step_clause(Some(parsed)).unwrap();
        assert_eq!(1, step.value());
        assert_eq!(TimeUnit::Day, step.unit());
    }

    #[test]
    fn test_simple_query() {
        let input = r#"GET AAPL.open, AAPL.volume / 1000
            FOR LAST 1 day
            STEP 1 hour
            "#;

        let parsed = QueryParser::parse(Rule::query, input)
            .expect("parse failed")
            .next()
            .unwrap();

        let query = parse_query(parsed).unwrap();
        dbg!(query);
    }
}
