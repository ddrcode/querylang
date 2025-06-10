# QueryServer POC

The POC introduces a simple, SQL-inspired languae for quering intraday stock data.
it returns the data as a table in either text or JSON format.


## Execution
1. Start mock GraphQl server: `cargo run --bin=gqlmock`
2. Start query server: `RUST_LOG=debug cargo run --bin=query`
3. Execute query with curl:
```
curl -X POST http://localhost:3000/query \
  -H "Content-Type: application/json" \
  -d '{"query": "GET APPL.max, GOOGL.open, GOOGL.volume FOR LAST 1 day STEP 1 hour", "format": "text"}'
```

It should produce a following output:

```
APPL.max  GOOGL.open  GOOGL.volume
------------------------------------
  110.11      127.60        147.92
  119.88      134.85        123.87
  112.28      119.55        136.43
  110.20      149.27        120.85
  139.13      129.35        134.41
  129.25      142.93        112.16
  102.11      134.66        118.67
  114.77      122.68        139.47
  135.58      132.73        118.37
  141.93      144.95        111.46
  101.42      140.55        136.18
  125.01      125.24        117.78
  116.98      146.49        139.76
  125.26      115.47        107.11
  133.15      126.95        108.11
  138.32      123.57        101.56
  146.34      101.46        100.54
  101.59      131.02        139.24
  149.43      142.97        124.79
  141.54      113.95        100.72
  112.35      118.02        146.82
  139.82      106.29        118.84
  124.54      104.52        146.21
  123.56      147.30        105.74
  100.76      101.57        147.19

```

and also log the GrapQl query as the server's debug:

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


## Main components and libraries used
- HTTP server - Axum, Tower, Tokio
- Language parser - Pest
- GrapQL client - graphql_client, reqwest
- Mock GraphQL server - Axum, Tokio, async_graphql


## The execution flow (optimistic path)
1. Query is sent to async HTTP server created with Axum and Tower
2. Query is parsed with Pest and produces `Query` structure
3. All data targets (symbols and metrics) are extracted from query and stored in QueryPlan structure
4. For each target a separate GraphQL query is generated
5. Symbol and metric data are collected (asynchronously) from the mock GQL server and stored in
   memory
6. Program goes through all expressions from the original query and executes them on real data
7. An output table is generated (as text or JSON)


## Query language

Th language allows for quering prices and volumes from the last `n` time units in given interval
(time series data). I.e. to retrieve GOOGL max price (per interval) for last 3 days in 2h intervals
the query should look like

```
GET GOOGL.max FOR LAST 3 days STEP 2 hours
```

The number of resulted rows is equal `last 72 hours / 2 hours step = 36`.
If there is no data for given interval the value is 0.

Multiple assets can be fetched, each will result in a separate column:

```
GET GOOGL.open, GOOGL.volume, APPl.avg FOR LAST 10 days STEP 1 day
```

The language allows for simple expressions to manipulate the output:

```
GET
    APPL.volume / 1000,
    (GOOGL.max - GOOGL.min) * 10
FOR LAST 30 days
STEP 1 day
```

The language is case-sensitive. It allows for splitting the code into multiple lines.


## Key source files

### Logic

- `query_handler.rs` - glues together entire logic - request handling, parsing, data quring and
output generation
- `parser.rs` - code parsing logic
- `gql_client.rs` - handling GraphQL queries
- `table_builder.rs` - data post-processing - table generation
- `column_builder.rs` - data post-processing - expressions execution
- `server.rs` - mock GraphQL server

### Models

- `query.rs` - query (DSL) representation after parsing
- `query_plan.rs` - symbol/metric representation for GraphQL querying
- `table.rs` - output data representation (with text formatting)


## Features

### Current

- async networking (request handling, GQL querying) with Tokio
-

### Easy to add

- parallel data postprocessing with Rayon
- query and output data caching
- streaming the output data (instead of HTTP response)

### Worth considering

- Compiling query language expressions into WebAssembly
