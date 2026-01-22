# Self-documenting Makefile for fhevm-relayer
.DEFAULT_GOAL := help
.PHONY: all help

# Default target
all: help

# Colors for help message
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)
MOCK_RPC_MANIFEST := test-support/ethereum_rpc_mock/Cargo.toml

# Cargo test command without sccache wrapper (avoids permission issues)
CARGO_TEST := RUSTC_WRAPPER= cargo test

##@ Testing
test-unit: ## Run library/unit tests only
	@$(CARGO_TEST) --lib
	@$(CARGO_TEST) --manifest-path $(MOCK_RPC_MANIFEST) --lib

test-build: ## Build all test binaries
	@echo "Building all test binaries..."
	@$(CARGO_TEST) --lib --no-run
	@$(CARGO_TEST) --manifest-path $(MOCK_RPC_MANIFEST) --no-run
	@$(CARGO_TEST) --test input_proof_v1_test --no-run
	@$(CARGO_TEST) --test public_decrypt_v1_test --no-run
	@$(CARGO_TEST) --test user_decrypt_v1_test --no-run
	@$(CARGO_TEST) --test input_proof_v2_test --no-run
	@$(CARGO_TEST) --test public_decrypt_v2_test --no-run
	@$(CARGO_TEST) --test user_decrypt_v2_test --no-run
	@$(CARGO_TEST) --test http_rate_limiter_test --no-run
	@$(CARGO_TEST) --test health_tests --no-run
	@$(CARGO_TEST) --test keyurl_test --no-run

test-run: ## Run pre-built test binaries
	@echo "Running pre-built test binaries..."
	@$(CARGO_TEST) --lib
	@$(CARGO_TEST) --manifest-path $(MOCK_RPC_MANIFEST) -- --test-threads=1
	@$(CARGO_TEST) --test input_proof_v1_test -- --test-threads=1
	@$(CARGO_TEST) --test public_decrypt_v1_test -- --test-threads=1
	@$(CARGO_TEST) --test user_decrypt_v1_test -- --test-threads=1
	@$(CARGO_TEST) --test input_proof_v2_test -- --test-threads=1
	@$(CARGO_TEST) --test public_decrypt_v2_test -- --test-threads=1
	@$(CARGO_TEST) --test user_decrypt_v2_test -- --test-threads=1
	@$(CARGO_TEST) --test http_rate_limiter_test -- --test-threads=1
	@$(CARGO_TEST) --test health_tests -- --test-threads=1
	@$(CARGO_TEST) --test keyurl_test -- --test-threads=1

test-long-running: ## Run long-running redundancy tests (requires --features long-running-tests)
	@echo "Running long-running redundancy tests..."
	@$(CARGO_TEST) --features long-running-tests test_listener_redundancy -- --test-threads=1

test-all: test-build test-run test-long-running ## Build and run all tests including long-running (unit + integration + long-running)

test-all-no-long-running: test-build test-run ## Build and run all tests except long-running tests (unit + integration only)

test-smoke-ok-all-v1: ## Run smoke test on ok (happy path) for v1 endpoints (user-decrypt, public-decrypt, input-proof) and health
	@echo "Building v1 tests..."
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v1_test --test user_decrypt_v1_test --test input_proof_v1_test --test health_tests --no-run
	@echo "Running v1 tests..."
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test user_decrypt_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test input_proof_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test health_tests -- --test-threads=4

test-smoke-ok-input-proof-v1: ## Run smoke test on ok (happy path) for input-proof v1
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test input_proof_v1_test test_success_single_request -- --test-threads=4

test-smoke-ok-user-decrypt-v1: ## Run smoke test on ok (happy path) for user-decrypt v1
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test user_decrypt_v1_test test_success_single_request -- --test-threads=4

test-smoke-ok-public-decrypt-v1: ## Run smoke test on ok (happy path) for public-decrypt v1
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v1_test test_success_single_request -- --test-threads=4

test-health-endpoints: ## Run health endpoint tests
	@$(CARGO_TEST) --test health_tests -- --test-threads=1

##@ V2 API Testing
test-smoke-ok-all-v2: ## Run smoke test on ok (happy path) for v2 endpoints (user-decrypt, public-decrypt, input-proof)
	@echo "Building v2 tests..."
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v2_test --test user_decrypt_v2_test --test input_proof_v2_test --no-run
	@echo "Running v2 tests..."
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v2_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test user_decrypt_v2_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test input_proof_v2_test test_success_single_request -- --test-threads=4

test-smoke-ok-input-proof-v2: ## Run smoke test on ok (happy path) for input-proof v2
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test input_proof_v2_test test_success_single_request -- --test-threads=4

test-smoke-ok-user-decrypt-v2: ## Run smoke test on ok (happy path) for user-decrypt v2
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test user_decrypt_v2_test test_success_single_request -- --test-threads=4

test-smoke-ok-public-decrypt-v2: ## Run smoke test on ok (happy path) for public-decrypt v2
	@RUSTFLAGS="-A warnings" $(CARGO_TEST) --test public_decrypt_v2_test test_success_single_request -- --test-threads=4

##@ Combined Testing
test-smoke-all-versions: ## Run smoke tests for both v1 and v2 endpoints
	@echo "Running v1 smoke tests..."
	@make test-smoke-ok-all-v1
	@echo "Running v2 smoke tests..."
	@make test-smoke-ok-all-v2

##@ Linting
clippy: ## Check code with clippy (no fixes)
	@cargo clippy --all-targets --all-features -- -D warnings
	@cargo clippy --all-targets --all-features --manifest-path $(MOCK_RPC_MANIFEST) -- -D warnings

clippy-fix: ## Fix code issues with clippy
	@cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
	@cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged --manifest-path $(MOCK_RPC_MANIFEST)

fmt: ## Check code formatting
	@cargo fmt --all -- --check
	@cargo fmt --all --manifest-path $(MOCK_RPC_MANIFEST) -- --check

fmt-fix: ## Fix code formatting
	@cargo fmt --all
	@cargo fmt --all --manifest-path $(MOCK_RPC_MANIFEST)

api-lint: ## Validate OpenAPI specification
	@npx @redocly/cli lint openapi-async-design.yaml

##@ SQL
sqlx-migrate: ## Run database migrations for relayer database
	@cd relayer-migrate && cargo sqlx migrate run --database-url "postgresql://postgres:postgres@localhost:5433/relayer_db"

sqlx-prepare: ## Prepare sqlx offline data for CI (run after schema changes)
	@cargo sqlx prepare --database-url "postgresql://postgres:postgres@localhost:5433/relayer_db" -- --all-targets

##@ Release
release-minor: ## Release minor version bump (no publish, no tag)
	@cargo release minor --no-publish --no-push --no-tag -x

release-patch: ## Release patch version bump (no publish, no tag)
	@cargo release patch --no-publish --no-push --no-tag -x

release-rc: ## Release candidate version bump (no publish, no tag)
	@cargo release rc --no-publish --no-push --no-tag -x


##@ Help
help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\n$(YELLOW)fhevm-relayer Test Suite$(RESET)\n\nUsage: make <target>\n\nAvailable targets:\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(YELLOW)%-35s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(WHITE)%s$(RESET)\n", substr($$0, 5) }' $(MAKEFILE_LIST)
