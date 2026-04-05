compile:
	cargo build --manifest-path Cargo.toml

build:
	cargo build --manifest-path Cargo.toml

build-release:
	cargo build --release --manifest-path Cargo.toml

build-core:
	cargo build --manifest-path core/Cargo.toml

build-program:
	cargo build --manifest-path program/Cargo.toml

build-test-input-program:
	cargo build --manifest-path test-input-program/Cargo.toml

build-confidential-token-program:
	cargo build --manifest-path confidential-token-program/Cargo.toml

build-sbf:
	cargo build-sbf --manifest-path program/Cargo.toml
	cargo build-sbf --manifest-path test-input-program/Cargo.toml
	cargo build-sbf --manifest-path confidential-token-program/Cargo.toml

test:
	cargo test --manifest-path Cargo.toml

test-core:
	cargo test --manifest-path core/Cargo.toml --test host_contracts

test-core-nocapture:
	cargo test --manifest-path core/Cargo.toml --test host_contracts -- --nocapture

test-program:
	cargo test --manifest-path Cargo.toml -p solana-host-contracts

test-no-run:
	cargo test --manifest-path Cargo.toml --no-run

clean:
	cargo clean --manifest-path Cargo.toml

localnet: build-sbf
	NO_DNA=1 anchor localnet --skip-build

localnet-live: build-sbf
	NO_DNA=1 anchor localnet --skip-build

localnet-bootstrap:
	cargo run --quiet --manifest-path local-cli/Cargo.toml -- bootstrap-localnet

localnet-test: build-sbf
	NO_DNA=1 anchor test --skip-build

localnet-test-existing: build-sbf
	NO_DNA=1 anchor test --skip-local-validator --skip-build

show-addresses:
	cat addresses/.env.host

.PHONY: compile build build-release build-core build-program build-test-input-program build-confidential-token-program build-sbf test test-core test-core-nocapture test-program test-no-run clean localnet localnet-live localnet-bootstrap localnet-test localnet-test-existing show-addresses
