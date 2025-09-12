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
	@cargo test

##@ Linting
clippy: ## Check code with clippy (no fixes)
	@cargo clippy --all-targets --all-features -- -D warnings

clippy-fix: ## Fix code issues with clippy
	@cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

fmt: ## Check code formatting
	@cargo fmt --all -- --check

fmt-fix: ## Fix code formatting
	@cargo fmt --all


##@ Help
help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\n$(YELLOW)fhevm-relayer Test Suite$(RESET)\n\nUsage: make <target>\n\nAvailable targets:\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(YELLOW)%-35s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(WHITE)%s$(RESET)\n", substr($$0, 5) }' $(MAKEFILE_LIST)
