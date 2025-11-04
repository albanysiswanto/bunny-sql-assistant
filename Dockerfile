# stage build
FROM rust:1.71 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# stage runtime
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/bunny-sql-assistant /usr/local/bin/bunny-sql-assistant
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/local/bin/bunny-sql-assistant"]
