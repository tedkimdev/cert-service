.PHONY: fmt lint test run check migrate db build run

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all

check: fmt lint test

migrate:
	cargo sqlx migrate run

db:
	docker compose -f ../docker-compose.yml up -d

build: migrate
	cargo build

run: migrate
	cargo run