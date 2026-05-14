APP_NAME=axum-microservice

.PHONY: dev build test fmt lint check docker-build docker-run

dev:
	cargo run

build:
	cargo build --release

test:
	cargo test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt lint test

docker-build:
	docker build -t $(APP_NAME):local .

docker-run:
	docker run --rm -p 8080:8080 --env-file .env $(APP_NAME):local
