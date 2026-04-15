.PHONY: test test-unit test-e2e test-e2e-amqp test-e2e-redis test-e2e-consumer clean-e2e-containers clippy

# ── Unit tests (no Docker required) ──────────────────────────────────────────
test-unit:
	cargo test --workspace

# ── E2E helpers ───────────────────────────────────────────────────────────────
# Named containers created by the shared-container pattern in e2e tests.
AMQP_CONTAINER := e2e-rabbitmq
REDIS_CONTAINER := e2e-redis

# Remove named e2e containers if they exist (idempotent).
clean-e2e-containers:
	@docker rm -f $(AMQP_CONTAINER) 2>/dev/null || true
	@docker rm -f $(REDIS_CONTAINER) 2>/dev/null || true

# ── broker (amqp) e2e ────────────────────────────────────────────────────────
# Runs the AMQP e2e suite and removes the shared container afterwards,
# even if the tests fail.
test-e2e-amqp: clean-e2e-containers
	cargo test -p broker --features amqp --test amqp_e2e -- --include-ignored; \
	$(MAKE) clean-e2e-containers

# ── broker (redis) e2e ───────────────────────────────────────────────────────
# Runs the Redis e2e suite and removes the shared container afterwards,
# even if the tests fail.
test-e2e-redis: clean-e2e-containers
	cargo test -p broker --features redis --test redis_e2e -- --include-ignored; \
	$(MAKE) clean-e2e-containers

# ── broker facade e2e ────────────────────────────────────────────────────────
# Runs the Broker facade e2e suite (high-level API: Broker → Publisher / ConsumerBuilder)
# and removes the shared container afterwards.
test-e2e-broker: clean-e2e-containers
	cargo test -p broker --features redis --test broker_e2e -- --include-ignored; \
	$(MAKE) clean-e2e-containers

# ── consumer-lib e2e ─────────────────────────────────────────────────────────
test-e2e-consumer: clean-e2e-containers
	cargo test -p consumer --test watch_e2e -- --include-ignored; \
	$(MAKE) clean-e2e-containers

# ── Run all e2e tests ─────────────────────────────────────────────────────────
test-e2e: test-e2e-amqp test-e2e-redis test-e2e-broker test-e2e-consumer

# ── Clippy (same flags as CI) ────────────────────────────────────────────────
clippy:
	cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings

# ── Run everything ────────────────────────────────────────────────────────────
test: test-unit test-e2e

# ── Run Format ───────────────────────────────────────────────────────────────
format:
	cargo fmt
