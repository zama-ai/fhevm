# kms-connector test helpers
#
# The DB-backed tests run against an already-running, shared Postgres server
# (each test creates its own uniquely-named database on it). These targets
# manage that server's lifecycle around `cargo nextest`.

DB_COMPOSE := docker compose -f connector-db/docker-compose.yml
DB_SERVICE := kms-connector-db

# Mirror the CI test environment so "passes locally" means "passes in CI".
TEST_ENV := SQLX_OFFLINE=true RUST_BACKTRACE=full

.PHONY: help db-up db-down test test-ci

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

db-up: ## Start the shared Postgres server (waits until healthy)
	@$(DB_COMPOSE) up -d --wait $(DB_SERVICE)

db-down: ## Stop the Postgres server and delete its volume
	@$(DB_COMPOSE) down -v

test: db-up ## Fast local loop: ensure DB is up, then run the tests (DB left running)
	@$(TEST_ENV) cargo nextest run

test-ci: ## Self-contained run: up -> test -> guaranteed teardown (mirrors CI)
	@$(DB_COMPOSE) up -d --wait $(DB_SERVICE)
	@$(TEST_ENV) cargo nextest run; status=$$?; \
	$(DB_COMPOSE) down -v; \
	exit $$status
