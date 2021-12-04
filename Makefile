.PHONY: fmt lint build lint-for-production test

fmt:
	cargo fmt

lint:
	cargo +nightly clippy -- -A clippy::print_literal

build: fmt lint
	cargo build

run: build
	./target/debug/github-actions-execution-time-collector \
		--o "kazune-br" \
		--r "github-actions-execution-time-collector" \
		--from "2021-11-6" \
		--to "2021-11-7"

lint-for-production:
	cargo clippy -- -D warnings

fix:
	cargo fix
	cargo +nightly clippy --fix -Z unstable-options

test:
	cargo test

release: fmt lint test build
	cargo build --release