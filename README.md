# Axum Rust Microservice

Production-style Rust microservice boilerplate using Axum, Tokio, Tower middleware, structured tracing, Docker, and integration tests.

## Features

- Axum 0.8-compatible app factory
- Versioned REST API under `/api/v1`
- Health and readiness endpoints
- Root service metadata endpoint
- Request ID middleware using `x-request-id`
- Tower HTTP tracing, timeout, and CORS layers
- Central JSON error responses
- Manual request validation for the example module
- Service/repository structure
- In-memory repository placeholder
- Outbound HTTP client placeholder
- Event publisher placeholder
- Background worker placeholder
- Integration tests with `tower::ServiceExt`
- Dockerfile, Docker Compose, Makefile, and GitHub Actions CI

## Run locally

```bash
cp .env.example .env
cargo run
```

Open:

```txt
http://localhost:8080
http://localhost:8080/api/v1/health
http://localhost:8080/api/v1/ready
```

## Endpoints

```txt
GET  /                     Service metadata
GET  /api/v1/health        Health check
GET  /api/v1/ready         Readiness check
GET  /api/v1/items         List items
POST /api/v1/items         Create item
GET  /api/v1/items/{id}    Get item by ID
```

## Example request

```bash
curl -X POST http://localhost:8080/api/v1/items \
  -H "content-type: application/json" \
  -H "x-request-id: demo-request-1" \
  -d '{"name":"Keyboard","description":"Mechanical keyboard","price":99.99}'
```

## Checks

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Or:

```bash
make check
```

## Docker

```bash
cp .env.example .env
docker compose up --build
```

## Environment variables

| Variable | Default | Description |
|---|---:|---|
| `APP_NAME` | `axum-microservice` | Service name |
| `APP_ENV` | `local` | Environment name |
| `HOST` | `0.0.0.0` | Bind host |
| `PORT` | `8080` | Bind port |
| `LOG_LEVEL` | `info` | Tracing filter |
| `REQUEST_TIMEOUT_SECONDS` | `15` | HTTP request timeout |

## Real-service next steps

Replace the in-memory repository with a durable datastore, replace the event publisher with Kafka/NATS/RabbitMQ/SNS/SQS, add auth middleware, and add metrics/OpenTelemetry when needed.
