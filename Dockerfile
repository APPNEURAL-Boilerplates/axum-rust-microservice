FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml rust-toolchain.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/axum-microservice /usr/local/bin/axum-microservice
ENV APP_NAME=axum-microservice \
    APP_ENV=production \
    HOST=0.0.0.0 \
    PORT=8080 \
    LOG_LEVEL=info \
    REQUEST_TIMEOUT_SECONDS=15
EXPOSE 8080
CMD ["axum-microservice"]
