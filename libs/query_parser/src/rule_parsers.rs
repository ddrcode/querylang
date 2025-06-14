use pest::{Parser, iterators::Pairs};

use super::{
    ParseError,
    builders::{build_expr, build_for_clause, build_query, build_step_clause},
    model::*,
    parser::{QueryParser, Rule},
};

type ParseResult<T> = Result<T, ParseError>;

/// Parses a full query string into a Query struct.
pub fn parse_query(src: &str) -> ParseResult<Query> {
    build_query(parse(src, Rule::query)?.next())
}

/// Parses a single expression into an Expr struct.
pub fn parse_expr(src: &str) -> ParseResult<Expr> {
    build_expr(parse(src, Rule::expr)?.next())
}

/// Parses a "FOR LAST" time clause into a TimeSpec struct.
pub fn parse_for_clause(src: &str) -> ParseResult<TimeSpec> {
    build_for_clause(parse(src, Rule::for_clause)?.next())
}

/// Parses a "STEP" clause into a TimeSpec struct.
pub fn parse_step_clause(src: &str) -> ParseResult<TimeSpec> {
    build_step_clause(parse(src, Rule::step_clause)?.next())
}

/// Internal helper to parse the source string using the given rule.
fn parse(input: &str, rule: Rule) -> ParseResult<Pairs<Rule>> {
    Ok(QueryParser::parse(rule, input)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use pest::iterators::Pair;

    /// Helper function useful for debugging a pair with all it inners
    #[allow(dead_code)]
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

    #[test]
    fn test_value_expr_parse() {
        let input = r"32";
        let expr = parse_expr(input).unwrap();
        assert_eq!(Expr::Value(32), expr);
    }

    #[test]
    fn test_symbol_expr_parse() {
        let input = r"AAPL.max";
        let expr = parse_expr(input).unwrap();
        assert_eq!(Expr::Data(SymbolMetric::new("AAPL", Metric::Max)), expr);
    }

    #[test]
    fn test_full_expr_parse() {
        let input = r"AAPL.volume / 1000";
        let expr = parse_expr(input).unwrap();
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
        let time = parse_for_clause(input).unwrap();
        assert_eq!(10, time.value());
        assert_eq!(TimeUnit::Day, time.unit());
    }

    #[test]
    fn test_step_clause() {
        let input = r"STEP 1 day";
        let step = parse_step_clause(input).unwrap();
        assert_eq!(1, step.value());
        assert_eq!(TimeUnit::Day, step.unit());
    }

    #[test]
    fn test_simple_query() {
        let input = r#"GET AAPL.open, AAPL.volume / 1000
            FOR LAST 1 day
            STEP 1 hour
            "#;

        let query = parse_query(input).unwrap();
        dbg!(query);
    }
}
