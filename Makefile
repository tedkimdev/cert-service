fmt:
	cargo fmt --all
lint:
	cargo clippy --all-targets --all-features -- -D warnings
test:
	cargo test --all
run:
	cargo run
check: fmt lint test