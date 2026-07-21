.PHONY: check fmt clippy test test-db sqlx-prepare db-create

SQLX_OFFLINE ?= true
export SQLX_OFFLINE

check:
	NO_DNA=1 cargo check --workspace --all-targets

fmt:
	NO_DNA=1 cargo fmt --all -- --check

clippy:
	NO_DNA=1 cargo clippy --workspace --all-targets -- -D warnings

test:
	NO_DNA=1 cargo test --workspace

# Real Postgres integration tests (ignored unless --ignored).
# Example:
#   export DATABASE_URL=postgres://work@127.0.0.1:55432/solana_proof_service
#   make test-db
test-db:
	@test -n "$$DATABASE_URL$$SOLANA_PROOF_TEST_DATABASE_URL" || (echo "Set DATABASE_URL or SOLANA_PROOF_TEST_DATABASE_URL"; exit 1)
	NO_DNA=1 SQLX_OFFLINE=true cargo test -p solana-proof-store --test store_postgres -- --ignored --nocapture

db-create:
	createdb solana_proof_service || true

# Regenerate compile-checked query metadata against a live DATABASE_URL.
sqlx-prepare:
	cd crates/solana-proof-store && NO_DNA=1 SQLX_OFFLINE=false cargo sqlx prepare -- --all-targets
