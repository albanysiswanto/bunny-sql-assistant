FROM rust:latest AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends libsqlite3-dev sqlite3 pkg-config ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install sqlx-cli --no-default-features --features sqlite

ENV DATABASE_URL="sqlite:db.sqlite"
RUN sqlite3 db.sqlite "" \
 && cargo sqlx prepare --database-url "$DATABASE_URL"

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/bunnysql /usr/local/bin/bunnysql
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/local/bin/bunnysql"]
