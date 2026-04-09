build:
	cargo build --manifest-path Cargo.toml

build-sbf:
	cargo build-sbf --manifest-path programs/solana-host-contracts/Cargo.toml
	cargo build-sbf --manifest-path programs/solana-test-input-program/Cargo.toml
	cargo build-sbf --manifest-path programs/solana-confidential-token-program/Cargo.toml

test:
	cargo test --manifest-path Cargo.toml

test-core:
	cargo test --manifest-path crates/core/Cargo.toml --test host_contracts

test-program:
	cargo test --manifest-path Cargo.toml -p solana-host-contracts

clean:
	cargo clean --manifest-path Cargo.toml

localnet: build-sbf
	NO_DNA=1 anchor localnet --skip-build

localnet-bootstrap:
	cargo run --quiet --manifest-path local-cli/Cargo.toml -- bootstrap-localnet

show-addresses:
	cat addresses/.env.host

.PHONY: build build-sbf test test-core test-program clean localnet localnet-bootstrap show-addresses
