.PHONY: fmt lint test run check migrate db build run certs

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	SQLX_OFFLINE=true cargo test --all

check: fmt lint test

migrate:
	cargo sqlx migrate run

db:
	docker compose -f ../docker-compose.yml up -d

build: migrate
	SQLX_OFFLINE=true cargo build

run: migrate
	SQLX_OFFLINE=true cargo run

certs:
	mkdir -p certs
	openssl req -x509 -newkey rsa:4096 \
		-keyout certs/key.pem \
		-out certs/cert.pem \
		-days 365 -nodes \
		-subj "/CN=localhost"