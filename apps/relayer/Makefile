# Self-documenting Makefile for fhevm-relayer
.DEFAULT_GOAL := help
.PHONY: all help

# Default target
all: help

# Colors for help message
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

##@ Testing
test-unit: ## Run library/unit tests only
	@cargo test --lib

test-all: ## Run all tests (unit + integration)
	@cargo test --lib
	@cargo test --test transaction_helper_test -- --test-threads=1
	@cargo test --test input_proof_v1_test -- --test-threads=1
	@cargo test --test public_decrypt_v1_test -- --test-threads=1
	@cargo test --test user_decrypt_v1_test -- --test-threads=1
	@cargo test --test http_rate_limiter_test -- --test-threads=1
	@cargo test --test health_tests -- --test-threads=1

test-smoke-ok-all-v1: ## Run smoke test on ok (happy path) for v1 endpoints (user-decrypt, public-decrypt, input-proof) and health
	@echo "Building v1 tests..."
	@RUSTFLAGS="-A warnings" cargo test --test public_decrypt_v1_test --test user_decrypt_v1_test --test input_proof_v1_test --test health_tests --no-run
	@echo "Running v1 tests..."
	@RUSTFLAGS="-A warnings" cargo test --test public_decrypt_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" cargo test --test user_decrypt_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" cargo test --test input_proof_v1_test test_success_single_request -- --test-threads=4
	@RUSTFLAGS="-A warnings" cargo test --test health_tests -- --test-threads=4

test-smoke-ok-input-proof-v1: ## Run smoke test on ok (happy path) for input-proof v1
	@RUSTFLAGS="-A warnings" cargo test --test input_proof_v1_test test_success_single_request -- --test-threads=4

test-smoke-ok-user-decrypt-v1: ## Run smoke test on ok (happy path) for user-decrypt v1
	@RUSTFLAGS="-A warnings" cargo test --test user_decrypt_v1_test test_success_single_request -- --test-threads=4

test-smoke-ok-public-decrypt-v1: ## Run smoke test on ok (happy path) for public-decrypt v1
	@RUSTFLAGS="-A warnings" cargo test --test public_decrypt_v1_test test_success_single_request -- --test-threads=4

test-health-endpoints: ## Run health endpoint tests
	@cargo test --test health_tests -- --test-threads=1

##@ Linting
clippy: ## Check code with clippy (no fixes)
	@cargo clippy --all-targets --all-features -- -D warnings

clippy-fix: ## Fix code issues with clippy
	@cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

fmt: ## Check code formatting
	@cargo fmt --all -- --check

fmt-fix: ## Fix code formatting
	@cargo fmt --all

api-lint: ## Validate OpenAPI specification
	@npx @redocly/cli lint openapi-async-design.yaml

##@ SQL
sqlx-migrate: ## Run database migrations for relayer database
	@cd relayer-migrate && cargo sqlx migrate run --database-url "postgresql://postgres:postgres@localhost:5433/relayer_db"

sqlx-prepare: ## Prepare sqlx offline data for CI (run after schema changes)
	@cargo sqlx prepare --database-url "postgresql://postgres:postgres@localhost:5433/relayer_db"

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
