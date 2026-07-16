.PHONY: check fmt clippy test

check:
	NO_DNA=1 cargo check --workspace --all-targets

fmt:
	NO_DNA=1 cargo fmt --all -- --check

clippy:
	NO_DNA=1 cargo clippy --workspace --all-targets -- -D warnings

test:
	NO_DNA=1 cargo test --workspace
