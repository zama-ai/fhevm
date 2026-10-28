# kms-connector test and build helpers
#
# The DB-backed tests run against an already-running, shared Postgres server
# (each test creates its own uniquely-named database on it). These targets
# manage that server's lifecycle around `cargo nextest`.

DB_COMPOSE := docker compose -f connector-db/docker-compose.yml
DB_SERVICE := kms-connector-db
DB_MIGRATION_SERVICE := kms-connector-db-migration

# Connection string for the local compose DB, used to regenerate the sqlx cache.
DATABASE_URL := postgresql://postgres:postgres@localhost:5432/kms-connector

# Mirror the CI test environment so "passes locally" means "passes in CI".
TEST_ENV := SQLX_OFFLINE=true RUST_BACKTRACE=full

# Foundry root holding the mock contracts the blockchain tests deploy at runtime.
CONTRACTS_ROOT := tests

# Runtime images built from Dockerfile.workspace (the builder stage is shared and cached).
DOCKER_TARGETS := gw-listener kms-worker tx-sender endpoint proxy
RUST_IMAGE_VERSION := $(shell sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)

.PHONY: help db-up db-down precompile-mocks test test-ci regen-sqlx-cache docker-build

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

db-up: ## Start the shared Postgres server (waits until healthy)
	@$(DB_COMPOSE) up -d --wait $(DB_SERVICE)

db-down: ## Stop the Postgres server and delete its volume
	@$(DB_COMPOSE) down -v

# The blockchain tests deploy the mocks with `forge create`, which compiles them
# lazily. Under nextest's parallelism, many `forge` processes otherwise race to
# install solc into the shared ~/.svm dir and exec it mid-write, failing with
# "Text file busy". Building once up front installs solc and warms the artifact
# cache so the concurrent deploys only deploy.
precompile-mocks: ## Compile the mock contracts once, serially, before the tests fan out
	@forge build --root $(CONTRACTS_ROOT)

test: db-up precompile-mocks ## Fast local loop: ensure DB is up, then run the tests (DB left running)
	@$(TEST_ENV) cargo nextest run

test-ci: precompile-mocks ## Self-contained run: up -> test -> guaranteed teardown (mirrors CI)
	@$(DB_COMPOSE) up -d --wait $(DB_SERVICE)
	@$(TEST_ENV) cargo nextest run; status=$$?; \
	$(DB_COMPOSE) down -v; \
	exit $$status

regen-sqlx-cache: ## Spin up DB + run migrations, regenerate the sqlx offline cache, then teardown
	@$(DB_COMPOSE) up -d --wait $(DB_SERVICE)
	@$(DB_COMPOSE) run --build --rm $(DB_MIGRATION_SERVICE); status=$$?; \
	if [ $$status -eq 0 ]; then \
		DATABASE_URL="$(DATABASE_URL)" cargo sqlx prepare --workspace -- --all-targets --all-features; status=$$?; \
	fi; \
	$(DB_COMPOSE) down -v; \
	exit $$status

# The zama-ai/kms git dependencies are private: the build reads a token with read access to it
# from the BLOCKCHAIN_ACTIONS_TOKEN BuildKit secret (never stored in a layer). Defaults to the
# `gh` CLI token.
docker-build: ## Build the connector images as kms-connector-<target>:local (TARGETS="kms-worker" for a subset)
	@export BLOCKCHAIN_ACTIONS_TOKEN="$${BLOCKCHAIN_ACTIONS_TOKEN:-$$(gh auth token 2>/dev/null)}"; \
	if [ -z "$$BLOCKCHAIN_ACTIONS_TOKEN" ]; then \
		echo "Needs read access to the private zama-ai/kms repository: set BLOCKCHAIN_ACTIONS_TOKEN or run 'gh auth login'." >&2; \
		exit 1; \
	fi; \
	for target in $(or $(TARGETS),$(DOCKER_TARGETS)); do \
		docker build -f Dockerfile.workspace --target $$target \
			--secret id=BLOCKCHAIN_ACTIONS_TOKEN,env=BLOCKCHAIN_ACTIONS_TOKEN \
			--build-arg RUST_IMAGE_VERSION=$(RUST_IMAGE_VERSION) \
			-t kms-connector-$$target:local .. || exit 1; \
	done
