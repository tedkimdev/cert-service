FROM rust:1.95-alpine AS chef
USER root
RUN apk add --no-cache musl-dev openssl-dev && cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin cert-service-practice

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

# non-root user
RUN useradd --system --uid 10001 appuser

WORKDIR /app
COPY --from=builder /app/target/release/cert-service-practice /app/cert-service
COPY --from=builder /app/migrations /app/migrations
COPY certs /app/certs

ENV RUST_LOG=info,tower_http=debug,sqlx=warn

EXPOSE 3000

HEALTHCHECK CMD curl --fail https://127.0.0.1:3000/health/ready -k || exit 1

USER appuser

ENTRYPOINT ["/app/cert-service"]