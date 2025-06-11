# QueryServer POC

A proof of concept (POC) introducing a simple, SQL-inspired language for querying intraday stock
data over a time range with custom step size.  Returns data as a table, in either text or JSON format.


## Quickstart

1. **Start the mock GraphQL server** (port 8001)
    ``` cargo run --bin=gqlmock ```

2. **Start the query server** (port 3000):
    ``` RUST_LOG=debug cargo run --bin=query ```

3. **Run a sample query:**
    ```
    curl -X POST http://localhost:3000/query \
      -H "Content-Type: application/json" \
      -d '{"query": "GET APPL.max, GOOGL.open, GOOGL.volume FOR LAST 1 day STEP 1 hour", "format": "text"}'
    ```

Sample text output:

```
time step  APPL.max  GOOGL.open  GOOGL.v...
---------------------------------------------
        0   114.86     109.11    2046.05
        1   153.54     110.65    2139.16
        2   115.33     143.61    1587.69
        3   140.78     149.52    2100.04
        4   164.45     144.66    1809.34
        5   148.83     137.22    2204.91
        6   159.66     138.62    2098.51
        7   133.88     136.67    2110.11
        8   159.40     117.53    1675.00
        9   164.24     126.36    1521.90
       10   114.29     108.58    1821.63
       11   116.58     121.65    2243.59
       12   151.10     149.86    2002.58
       13   164.82     103.26    2052.62
       14   159.33     100.71    2150.00
       15   118.98     107.39    2103.16
       16   139.43     137.34    1861.30
       17   126.42     119.04    1788.59
       18   114.70     109.75    2184.75
       19   134.39     103.38    2212.73
       20   135.20     127.30    1957.11
       21   118.40     106.30    1513.93
       22   147.36     135.15    1863.22
       23   146.75     110.89    1745.25

```

Generated GraphQL query example (server log):

```
2025-06-10T12:25:32.604435Z DEBUG request{method=POST uri=/query version=HTTP/1.1}: query::query_engine::gql_client: GraphQL Query payload: {
  "variables": {
    "symbol": "GOOGL",
    "metrics": [
      "open",
      "volume"
    ],
    "from": "2025-06-09T12:25:32.603833+00:00",
    "to": "2025-06-10T12:25:32.603833+00:00",
    "step": "1"
  },
  "query": "query GetMetrics($symbol: String!, $metrics: [String!]!, $from: String!, $to: String!, $step: String!) {\n  getMetrics(symbol: $symbol, metrics: $metrics, from: $from, to: $to, step: $step) {\n    timestamp\n    values {\n      metric\n      value\n    }\n  }\n}\n\n",
  "operationName": "GetMetrics"
}
```

- The `format` parameter accepts `"json"` or `"text"` (default is `"text"`).
- A web-based GraphQL playground is also available at `http://localhost:8001`.



## Query language with examples

The language allows for querying prices and volumes from the last `n` time units in given interval (time series data).
I.e. to retrieve GOOGL max price (per interval) for last 3 days in 2h intervals the query should look like

```GET GOOGL.max FOR LAST 3 days STEP 2 hours```

The number of resulted rows is equal to `last 72 hours / 2 hours step = 36`.
The assumption is that if there is no data for given interval the value is 0.


### Multiple Assets, Multiple Metrics

```
GET TSLA.open, AAPL.volume FOR LAST 1 day STEP 1 hour
```

Result: TSLA opening price and volume of AAPL for each hour in the last day.


### Expressions

```
GET
    APPL.volume / 1000,
    (GOOGL.max - GOOGL.min) * 10
FOR LAST 30 days
STEP 1 day
```

### Rules / assumptions

- If no data for an interval, the value is 0.
- Multiple metrics for the same asset produce single GQL query
- Repeated assets in multiple expressions produce only a single GQL query
- Case-sensitive; multi-line code.
- DSL grammar is defined in [`query.pest`](src/adapter/parser/query.pest) file



## Architecture Overview

### Major components and crates

- **HTTP server**: Axum, Tower, Tokio
- **Language parser**: Pest
- **GraphQL client**: graphql_client, reqwest
- **Mock GraphQL server**: Axum, Tokio, async_graphql

### Execution flow (for optimistic path)

1. Query is sent to async HTTP server (Axum, Tower)
2. Query is parsed with Pest, producing a `Query` structure
3. Data targets (symbols and metrics) are extracted into a `QueryPlan`
4. For each target, a separate GraphQL query is generated and sent. No duplicated data fetches are
   guaranteed
5. Data is collected (async) from the mock server and stored in memory
6. Each query expression is executed on real data
7. Output table is generated (as text or JSON)

<img width="419" alt="image" src="https://github.com/user-attachments/assets/ad8fc9c3-5703-43f3-beb0-413734a4081e" />


## Features

### Current

- Async networking with Tokio (requests and GQL querying)
- Parallel column generation (each column in a separate task)
- Fast parsing engine
- No panics/unwraps, robust error handling

### Easy to add

- parallel data post-processing with Rayon,
- query/output caching
- web client

### Further considerations
- Compile query language expressions to WebAssembly
- streaming output data


## Key Source Files

- [`query_handler.rs`](src/api/query_handler.rs) - handles query request
- [`parser.rs`](src/adapter/parser/parser.rs) - query parsing logic (grammar file:
[`query.pest`](src/adapter/parser/query.pest))
- [`metrics_repository_gql.rs`](src/repository/metrics_repository_gql.rs) - GraphQL client
- [`query_service.rs`](src/service/query_service.rs) - main service (glue logic)
- [`server.rs`](src/gql_server/server.rs) - mock GraphQL server

Models:
- [`query.rs`](src/domain/query.rs) - query (DSL) representation after parsing
- [`query_plan.rs`](src/shared/query_plan.rs) - symbol/metric representation for GQL querying
- [`table.rs`](src/domain/table.rs) - output data representation and formatting

## Folder structure

```
src/
├── api/                # API endpoint handlers
├── service/            # System services (called by handlers)
├── repository/         # Data access handlers and traits
├── adapter/parser/     # Query DSL, grammar, parser code
├── domain/             # Domain models (Query, Table, etc)
├── shared/             # DTO's, non-domain structs, etc
├── error/              # Error definitions
├── main.rs             # Entrypoint for query server
├── config.rs           # Configuration options
└── ...
```
