.PHONY: fmt lint build lint-for-production test

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -A clippy::print_literal

lint-for-production:
	cargo clippy -- -D warnings

build: fmt lint
	cargo build

run-debug: build
	./target/debug/github-actions-execution-time-collector \
		--o="kazune-br" \
		--r="github-actions-execution-time-collector" \
		--from="2022-12-1" \
		--to="2022-12-31"

release: fmt lint-for-production test
	cargo build --release

run: release
	./target/release/github-actions-execution-time-collector \
		--o="kazune-br" \
		--r="github-actions-execution-time-collector" \
		--from="2022-12-1" \
		--to="2022-12-31"

switch-tool-chain:
	rustup default $$(printf "stable\nnightly" | fzf)