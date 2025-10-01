# FHEVM Master Makefile
# Comprehensive build system for the entire FHEVM project
# 
# Usage:
#   make help          - Show available targets
#   make build         - Build all components
#   make test          - Run all tests
#   make clean         - Clean all build artifacts
#   make docker        - Build all Docker images
#   make install       - Install all dependencies
#   make lint          - Run linting on all projects
#   make format        - Format all code

# Default target
.DEFAULT_GOAL := help

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
NC := \033[0m # No Color

# Project configuration
PROJECT_NAME := fhevm
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
GIT_COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# Build configuration
BUILD_DIR := build
DOCKER_BUILD_DIR := docker-build
PARALLEL_JOBS := $(shell nproc 2>/dev/null || echo 4)

# Project directories
GATEWAY_CONTRACTS := gateway-contracts
HOST_CONTRACTS := host-contracts
LIBRARY_SOLIDITY := library-solidity
PROTOCOL_CONTRACTS := protocol-contracts
TEST_SUITE_E2E := test-suite/e2e
TEST_SUITE_BENCHMARKS := test-suite/benchmarks
COPROCESSOR := coprocessor
KMS_CONNECTOR := kms-connector

# All project directories
PROJECTS := $(GATEWAY_CONTRACTS) $(HOST_CONTRACTS) $(LIBRARY_SOLIDITY) $(PROTOCOL_CONTRACTS)
TEST_PROJECTS := $(TEST_SUITE_E2E) $(TEST_SUITE_BENCHMARKS)
RUST_PROJECTS := $(COPROCESSOR) $(KMS_CONNECTOR)

# Docker images
DOCKER_IMAGES := gateway-contracts host-contracts coprocessor kms-connector

# Phony targets
.PHONY: help build test clean docker install lint format check deps update-deps \
        build-gateway build-host build-library build-protocol \
        test-gateway test-host test-library test-protocol \
        clean-gateway clean-host clean-library clean-protocol \
        docker-gateway docker-host docker-coprocessor docker-kms \
        lint-gateway lint-host lint-library lint-protocol \
        format-gateway format-host format-library format-protocol \
        check-gateway check-host check-library check-protocol \
        deps-gateway deps-host deps-library deps-protocol \
        rust-build rust-test rust-clean \
        parallel-build parallel-test parallel-clean

# Help target
help: ## Show this help message
	@echo "$(CYAN)FHEVM Build System$(NC)"
	@echo "$(CYAN)==================$(NC)"
	@echo ""
	@echo "$(GREEN)Available targets:$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(GREEN)Project Info:$(NC)"
	@echo "  Version: $(VERSION)"
	@echo "  Build Time: $(BUILD_TIME)"
	@echo "  Git Commit: $(GIT_COMMIT)"
	@echo "  Parallel Jobs: $(PARALLEL_JOBS)"

# Main build targets
build: deps ## Build all components
	@echo "$(GREEN)Building all components...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) build-gateway build-host build-library build-protocol
	@$(MAKE) -j$(PARALLEL_JOBS) rust-build
	@echo "$(GREEN)✓ All components built successfully$(NC)"

test: build ## Run all tests
	@echo "$(GREEN)Running all tests...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) test-gateway test-host test-library test-protocol
	@$(MAKE) -j$(PARALLEL_JOBS) rust-test
	@echo "$(GREEN)✓ All tests passed$(NC)"

clean: ## Clean all build artifacts
	@echo "$(GREEN)Cleaning all build artifacts...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) clean-gateway clean-host clean-library clean-protocol
	@$(MAKE) -j$(PARALLEL_JOBS) rust-clean
	@rm -rf $(BUILD_DIR) $(DOCKER_BUILD_DIR)
	@echo "$(GREEN)✓ All build artifacts cleaned$(NC)"

docker: ## Build all Docker images
	@echo "$(GREEN)Building all Docker images...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) docker-gateway docker-host docker-coprocessor docker-kms
	@echo "$(GREEN)✓ All Docker images built$(NC)"

install: ## Install all dependencies
	@echo "$(GREEN)Installing all dependencies...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) deps-gateway deps-host deps-library deps-protocol
	@echo "$(GREEN)✓ All dependencies installed$(NC)"

lint: ## Run linting on all projects
	@echo "$(GREEN)Running linting on all projects...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) lint-gateway lint-host lint-library lint-protocol
	@echo "$(GREEN)✓ All linting completed$(NC)"

format: ## Format all code
	@echo "$(GREEN)Formatting all code...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) format-gateway format-host format-library format-protocol
	@echo "$(GREEN)✓ All code formatted$(NC)"

check: lint ## Run all checks (lint + format check)
	@echo "$(GREEN)Running all checks...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) check-gateway check-host check-library check-protocol
	@echo "$(GREEN)✓ All checks passed$(NC)"

deps: ## Install all dependencies
	@echo "$(GREEN)Installing root dependencies...$(NC)"
	@npm install
	@echo "$(GREEN)Installing project dependencies...$(NC)"
	@$(MAKE) install

update-deps: ## Update all dependencies
	@echo "$(GREEN)Updating all dependencies...$(NC)"
	@npm update
	@for project in $(PROJECTS); do \
		echo "$(BLUE)Updating dependencies for $$project...$(NC)"; \
		cd $$project && npm update && cd ..; \
	done
	@echo "$(GREEN)✓ All dependencies updated$(NC)"

# Individual project build targets
build-gateway: ## Build gateway contracts
	@echo "$(BLUE)Building gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && $(MAKE) compile
	@echo "$(GREEN)✓ Gateway contracts built$(NC)"

build-host: ## Build host contracts
	@echo "$(BLUE)Building host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npm run compile
	@echo "$(GREEN)✓ Host contracts built$(NC)"

build-library: ## Build library solidity
	@echo "$(BLUE)Building library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm run compile
	@echo "$(GREEN)✓ Library solidity built$(NC)"

build-protocol: ## Build protocol contracts
	@echo "$(BLUE)Building protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm run compile
	@echo "$(GREEN)✓ Protocol contracts built$(NC)"

# Individual project test targets
test-gateway: build-gateway ## Test gateway contracts
	@echo "$(BLUE)Testing gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && $(MAKE) test
	@echo "$(GREEN)✓ Gateway contracts tested$(NC)"

test-host: build-host ## Test host contracts
	@echo "$(BLUE)Testing host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npm test
	@echo "$(GREEN)✓ Host contracts tested$(NC)"

test-library: build-library ## Test library solidity
	@echo "$(BLUE)Testing library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm test
	@echo "$(GREEN)✓ Library solidity tested$(NC)"

test-protocol: build-protocol ## Test protocol contracts
	@echo "$(BLUE)Testing protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm test
	@echo "$(GREEN)✓ Protocol contracts tested$(NC)"

# Individual project clean targets
clean-gateway: ## Clean gateway contracts
	@echo "$(BLUE)Cleaning gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && $(MAKE) clean
	@echo "$(GREEN)✓ Gateway contracts cleaned$(NC)"

clean-host: ## Clean host contracts
	@echo "$(BLUE)Cleaning host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npx hardhat clean
	@echo "$(GREEN)✓ Host contracts cleaned$(NC)"

clean-library: ## Clean library solidity
	@echo "$(BLUE)Cleaning library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npx hardhat clean
	@echo "$(GREEN)✓ Library solidity cleaned$(NC)"

clean-protocol: ## Clean protocol contracts
	@echo "$(BLUE)Cleaning protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npx hardhat clean
	@echo "$(GREEN)✓ Protocol contracts cleaned$(NC)"

# Docker build targets
docker-gateway: ## Build gateway contracts Docker image
	@echo "$(BLUE)Building gateway contracts Docker image...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && docker build -t fhevm-gateway:$(VERSION) .
	@echo "$(GREEN)✓ Gateway contracts Docker image built$(NC)"

docker-host: ## Build host contracts Docker image
	@echo "$(BLUE)Building host contracts Docker image...$(NC)"
	@cd $(HOST_CONTRACTS) && docker build -t fhevm-host:$(VERSION) .
	@echo "$(GREEN)✓ Host contracts Docker image built$(NC)"

docker-coprocessor: ## Build coprocessor Docker image
	@echo "$(BLUE)Building coprocessor Docker image...$(NC)"
	@cd $(COPROCESSOR) && docker build -t fhevm-coprocessor:$(VERSION) .
	@echo "$(GREEN)✓ Coprocessor Docker image built$(NC)"

docker-kms: ## Build KMS connector Docker image
	@echo "$(BLUE)Building KMS connector Docker image...$(NC)"
	@cd $(KMS_CONNECTOR) && docker build -t fhevm-kms:$(VERSION) .
	@echo "$(GREEN)✓ KMS connector Docker image built$(NC)"

# Individual project lint targets
lint-gateway: ## Lint gateway contracts
	@echo "$(BLUE)Linting gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && npm run lint || echo "$(YELLOW)⚠ No lint script in gateway contracts$(NC)"

lint-host: ## Lint host contracts
	@echo "$(BLUE)Linting host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npm run lint
	@echo "$(GREEN)✓ Host contracts linted$(NC)"

lint-library: ## Lint library solidity
	@echo "$(BLUE)Linting library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm run lint
	@echo "$(GREEN)✓ Library solidity linted$(NC)"

lint-protocol: ## Lint protocol contracts
	@echo "$(BLUE)Linting protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm run lint || echo "$(YELLOW)⚠ No lint script in protocol contracts$(NC)"

# Individual project format targets
format-gateway: ## Format gateway contracts
	@echo "$(BLUE)Formatting gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && npm run format || npx prettier --write .
	@echo "$(GREEN)✓ Gateway contracts formatted$(NC)"

format-host: ## Format host contracts
	@echo "$(BLUE)Formatting host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npm run format
	@echo "$(GREEN)✓ Host contracts formatted$(NC)"

format-library: ## Format library solidity
	@echo "$(BLUE)Formatting library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm run format
	@echo "$(GREEN)✓ Library solidity formatted$(NC)"

format-protocol: ## Format protocol contracts
	@echo "$(BLUE)Formatting protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm run format || npx prettier --write .
	@echo "$(GREEN)✓ Protocol contracts formatted$(NC)"

# Individual project check targets
check-gateway: lint-gateway ## Check gateway contracts
	@echo "$(BLUE)Checking gateway contracts...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && npm run format:check || npx prettier --check .
	@echo "$(GREEN)✓ Gateway contracts checked$(NC)"

check-host: lint-host ## Check host contracts
	@echo "$(BLUE)Checking host contracts...$(NC)"
	@cd $(HOST_CONTRACTS) && npm run format:check
	@echo "$(GREEN)✓ Host contracts checked$(NC)"

check-library: lint-library ## Check library solidity
	@echo "$(BLUE)Checking library solidity...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm run format:check
	@echo "$(GREEN)✓ Library solidity checked$(NC)"

check-protocol: lint-protocol ## Check protocol contracts
	@echo "$(BLUE)Checking protocol contracts...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm run format:check || npx prettier --check .
	@echo "$(GREEN)✓ Protocol contracts checked$(NC)"

# Individual project dependency targets
deps-gateway: ## Install gateway contracts dependencies
	@echo "$(BLUE)Installing gateway contracts dependencies...$(NC)"
	@cd $(GATEWAY_CONTRACTS) && npm install
	@echo "$(GREEN)✓ Gateway contracts dependencies installed$(NC)"

deps-host: ## Install host contracts dependencies
	@echo "$(BLUE)Installing host contracts dependencies...$(NC)"
	@cd $(HOST_CONTRACTS) && npm install
	@echo "$(GREEN)✓ Host contracts dependencies installed$(NC)"

deps-library: ## Install library solidity dependencies
	@echo "$(BLUE)Installing library solidity dependencies...$(NC)"
	@cd $(LIBRARY_SOLIDITY) && npm install
	@echo "$(GREEN)✓ Library solidity dependencies installed$(NC)"

deps-protocol: ## Install protocol contracts dependencies
	@echo "$(BLUE)Installing protocol contracts dependencies...$(NC)"
	@cd $(PROTOCOL_CONTRACTS) && npm install
	@echo "$(GREEN)✓ Protocol contracts dependencies installed$(NC)"

# Rust project targets
rust-build: ## Build all Rust projects
	@echo "$(GREEN)Building Rust projects...$(NC)"
	@for project in $(RUST_PROJECTS); do \
		echo "$(BLUE)Building $$project...$(NC)"; \
		cd $$project && cargo build --release && cd ..; \
	done
	@echo "$(GREEN)✓ All Rust projects built$(NC)"

rust-test: ## Test all Rust projects
	@echo "$(GREEN)Testing Rust projects...$(NC)"
	@for project in $(RUST_PROJECTS); do \
		echo "$(BLUE)Testing $$project...$(NC)"; \
		cd $$project && cargo test && cd ..; \
	done
	@echo "$(GREEN)✓ All Rust projects tested$(NC)"

rust-clean: ## Clean all Rust projects
	@echo "$(GREEN)Cleaning Rust projects...$(NC)"
	@for project in $(RUST_PROJECTS); do \
		echo "$(BLUE)Cleaning $$project...$(NC)"; \
		cd $$project && cargo clean && cd ..; \
	done
	@echo "$(GREEN)✓ All Rust projects cleaned$(NC)"

# Parallel execution helpers
parallel-build: ## Build all projects in parallel
	@echo "$(GREEN)Building all projects in parallel...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) build-gateway build-host build-library build-protocol rust-build

parallel-test: ## Test all projects in parallel
	@echo "$(GREEN)Testing all projects in parallel...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) test-gateway test-host test-library test-protocol rust-test

parallel-clean: ## Clean all projects in parallel
	@echo "$(GREEN)Cleaning all projects in parallel...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) clean-gateway clean-host clean-library clean-protocol rust-clean

# Development helpers
dev-setup: deps ## Setup development environment
	@echo "$(GREEN)Setting up development environment...$(NC)"
	@mkdir -p $(BUILD_DIR) $(DOCKER_BUILD_DIR)
	@echo "$(GREEN)✓ Development environment ready$(NC)"

dev-build: dev-setup build ## Build for development
	@echo "$(GREEN)Development build complete$(NC)"

dev-test: dev-build test ## Test for development
	@echo "$(GREEN)Development test complete$(NC)"

# CI/CD helpers
ci-build: ## Build for CI/CD
	@echo "$(GREEN)CI build starting...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) build
	@echo "$(GREEN)✓ CI build complete$(NC)"

ci-test: ci-build ## Test for CI/CD
	@echo "$(GREEN)CI test starting...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) test
	@echo "$(GREEN)✓ CI test complete$(NC)"

ci-lint: ## Lint for CI/CD
	@echo "$(GREEN)CI lint starting...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) lint
	@echo "$(GREEN)✓ CI lint complete$(NC)"

# Production helpers
prod-build: ## Build for production
	@echo "$(GREEN)Production build starting...$(NC)"
	@$(MAKE) -j$(PARALLEL_JOBS) build
	@$(MAKE) -j$(PARALLEL_JOBS) docker
	@echo "$(GREEN)✓ Production build complete$(NC)"

# Utility targets
info: ## Show build information
	@echo "$(CYAN)FHEVM Build Information$(NC)"
	@echo "$(CYAN)========================$(NC)"
	@echo "Project: $(PROJECT_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Build Time: $(BUILD_TIME)"
	@echo "Git Commit: $(GIT_COMMIT)"
	@echo "Parallel Jobs: $(PARALLEL_JOBS)"
	@echo "Projects: $(PROJECTS)"
	@echo "Test Projects: $(TEST_PROJECTS)"
	@echo "Rust Projects: $(RUST_PROJECTS)"

version: ## Show version information
	@echo "$(VERSION)"

# Include project-specific makefiles if they exist
-include $(GATEWAY_CONTRACTS)/Makefile.local
-include $(HOST_CONTRACTS)/Makefile.local
-include $(LIBRARY_SOLIDITY)/Makefile.local
-include $(PROTOCOL_CONTRACTS)/Makefile.local
