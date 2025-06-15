FROM rust:1.86 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin query-api

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/query-api .
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
CMD ["./query-api"]
