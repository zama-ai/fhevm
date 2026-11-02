# Development tooling for relayer-http against the local fhevm stack. Run from anywhere:
#   make -C relayer-http/dev <rule>       (see README.md)
ROOT    := $(abspath ..)
# relayer-http ports, passed to the scripts; override on the command line (make RELAYER_HTTP_RUN_PORT=5002 run).
# RELAYER_HTTP_PORT: relayer-http in the stack (./fhevm-cli up), as in test-suite/fhevm/src/layout.ts.
# RELAYER_HTTP_RUN_PORT: relayer-http on the host (make run); its forwarders take the next ports.
export RELAYER_HTTP_PORT     ?= 3002
export RELAYER_HTTP_RUN_PORT ?= 4002
# GET /metrics on the host, both scraped by `make start-monitoring`. Uncommon ports, next to the API ports above.
# RELAYER_HTTP_STACK_METRICS_PORT: the stack's relayer (instance `fhevm-cli`), as published in
# test-suite/fhevm/docker-compose/relayer-http-docker-compose.yml.
# RELAYER_HTTP_METRICS_PORT: the relayer `make run` starts (instance `host`).
export RELAYER_HTTP_STACK_METRICS_PORT ?= 3092
export RELAYER_HTTP_METRICS_PORT       ?= 4092
# RELAYER_HTTP_MONITORING_HOST: the address Prometheus, Grafana and Loki listen on. Loopback by default: Grafana runs
# anonymously, so a LAN address (to browse from another machine) exposes it to that network.
export RELAYER_HTTP_MONITORING_HOST    ?= 127.0.0.1
# Loki, published for `make check-dashboards`.
export RELAYER_HTTP_LOKI_PORT          ?= 3310
# Where this machine reaches them: loopback when they listen on every interface.
MONITORING_URL_HOST := $(if $(filter 0.0.0.0,$(RELAYER_HTTP_MONITORING_HOST)),127.0.0.1,$(RELAYER_HTTP_MONITORING_HOST))
# `make upgrade-fhevm-cli-relayer`: the stack's state (rendered config and env; FHEVM_STATE_DIR when fhevm-cli runs from another
# checkout) and the tag of the image built from this checkout.
export FHEVM_STATE_DIR ?= $(abspath $(ROOT)/../.fhevm)
RELAYER_HTTP_LOCAL_TAG ?= local
STACK_COMPOSE := docker compose -p fhevm -f $(abspath $(ROOT)/../test-suite/fhevm/docker-compose/relayer-http-docker-compose.yml) \
	$(addprefix -f ,$(wildcard $(FHEVM_STATE_DIR)/runtime/compose/relayer-http.yml))
# Pinned like the listener's monitoring stack; promtool ships in the Prometheus image.
PROMTOOL := docker run --rm -i --entrypoint promtool prom/prometheus:v3.9.1
PROMTOOL_MONITORING := docker run --rm -v $(ROOT)/monitoring:/monitoring:ro -w /monitoring --entrypoint promtool prom/prometheus:v3.9.1
CLIPPY  := cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings

.PHONY: help fmt fmt-fix clippy test test-aws check check-alerts references build run down upgrade-fhevm-cli-relayer \
	public-decrypt user-decrypt load check-metrics start-monitoring stop-monitoring check-dashboards \
	pad-stack pad-connectors pad-run pad-decrypt

help: ## this list
	@grep -E '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{ printf "  %-26s %s\n", $$1, $$2 }'

# --- code -------------------------------------------------------------------------------------------------------
fmt: ## cargo fmt -- --check (not --all: it would follow the kms-connector-api path dependency into generated bindings)
	cd $(ROOT) && cargo fmt -- --check
fmt-fix: ## cargo fmt
	cd $(ROOT) && cargo fmt
clippy: ## the clippy line every change must pass
	cd $(ROOT) && $(CLIPPY)
test: ## cargo nextest run --workspace (the emulator tests are ignored; see test-aws)
	cd $(ROOT) && cargo nextest run --workspace
test-aws: ## the AWS KMS signer as deployed (IRSA credentials), against an AWS emulator (Floci) started for the run
	$(ROOT)/dev/scripts/test-aws.sh
check: fmt clippy test check-alerts ## fmt + clippy + test + the alert rule tests
build: ## cargo build -p relayer-http
	cd $(ROOT) && cargo build -p relayer-http

# --- local stack ------------------------------------------------------------------------------------------------
run: ## the relayer on the host (RELAYER_HTTP_RUN_PORT, default 4002) with the stack's config; logs here and in target/relayer-run.log
	scripts/relayer-run.sh
down: ## remove the forwarders `make run` leaves running
	@ids="$$(docker ps -aq --filter name='^relayer-http-fwd-')"; [ -z "$$ids" ] || docker rm -f $$ids
upgrade-fhevm-cli-relayer: ## build relayer-http's image from this checkout and swap it into the running stack (fhevm-cli up restores the published one)
	@# Plain progress: a readable build log under make. The base images need registry logins first (README): without one,
	@# the cgr.dev credential helper waits for an interactive login until BuildKit times out.
	cd $(ROOT)/.. && docker build --progress=plain -f relayer-http/Dockerfile \
		--build-arg RUST_IMAGE_VERSION=$$(sed -nE 's/^channel = "([^"]+)"/\1/p' relayer-http/rust-toolchain.toml) \
		-t ghcr.io/zama-ai/fhevm/relayer-http:$(RELAYER_HTTP_LOCAL_TAG) .
	RELAYER_HTTP_VERSION=$(RELAYER_HTTP_LOCAL_TAG) $(STACK_COMPOSE) up -d --no-deps --force-recreate --pull never relayer-http

# --- observability (OBSERVABILITY.md) -----------------------------------------------------------------------------
check-alerts: ## promtool check + unit tests of monitoring/alerts.yaml (docker, the Prometheus image)
	$(PROMTOOL_MONITORING) check rules alerts.yaml
	$(PROMTOOL_MONITORING) test rules alerts_test.yaml
references: ## regenerate monitoring/{summary_log,metrics,labels}.md from the code
	cd $(ROOT) && UPDATE_REFERENCES=1 cargo nextest run -p relayer-http references_are_current
check-metrics: ## promtool check metrics on the running relayer's /metrics (make run first; RELAYER_HTTP_METRICS_PORT)
	curl -fsS http://127.0.0.1:$(RELAYER_HTTP_METRICS_PORT)/metrics | $(PROMTOOL) check metrics
start-monitoring: ## local Prometheus (metrics, alerts) + Loki/Alloy (relayer logs) + Grafana with the dashboards (:3300, no login; RELAYER_HTTP_MONITORING_HOST)
	@# Recreated every time: compose keeps a running container when only its inline Prometheus config changed.
	docker compose -f monitoring/docker-compose.yml up -d --force-recreate
	@echo "Grafana: http://$(MONITORING_URL_HOST):$${RELAYER_HTTP_GRAFANA_PORT:-3300}/d/relayer-http"
	@echo "Prometheus targets: http://$(MONITORING_URL_HOST):$${RELAYER_HTTP_PROMETHEUS_PORT:-9390}/targets"
stop-monitoring: ## stop and remove the local monitoring stack (Prometheus, Loki, Alloy, Grafana; their data is not kept)
	docker compose -f monitoring/docker-compose.yml down
check-dashboards: ## every dashboard query through the local Grafana; the log panels over a scenario the code logs (start-monitoring first)
	cd $(ROOT) && RELAYER_HTTP_GRAFANA_URL=http://$(MONITORING_URL_HOST):$${RELAYER_HTTP_GRAFANA_PORT:-3300} \
		RELAYER_HTTP_LOKI_URL=http://$(MONITORING_URL_HOST):$(RELAYER_HTTP_LOKI_PORT) \
		cargo nextest run -p relayer-http --run-ignored only -E 'test(/monitoring::grafana/)'

# --- flows (e2e debug container) ----------------------------------------------------------------------------------
public-decrypt: ## deploy HTTPPublicDecrypt and POST /v4/public-decrypt (ATTEMPTS, RETRY_DELAY_MS, RELAYER_HTTP_URL)
	scripts/e2e.sh public-decrypt.ts
user-decrypt: ## deploy UserDecrypt, sign the unified request, POST /v4/user-decrypt (+ MIN_SHARES)
	scripts/e2e.sh user-decrypt.ts
load: ## k6 traffic mix: PROFILE=normal (1 per 5 s) or load (RATE/s, default 100), DURATION, TARGET=fhevm-cli|host
	scripts/load.sh

# --- pad decryption (RFC-030): the SDK's cleartext stack on anvil, mock KMS connectors; no fhevm stack needed ------
pad-stack: ## anvil :8544 with the v1.0.0 cleartext host contracts (terminal 1)
	scripts/pad.sh stack
pad-connectors: ## mock KMS connectors 127.0.0.1:8091..8094 serving /v1/pad-decrypt from that stack (terminal 2)
	scripts/pad.sh connectors
pad-run: ## the relayer with dev/config/pad-cleartext.yaml (terminal 3)
	scripts/pad.sh run
pad-decrypt: ## sign and POST /v4/pad-decrypt, check every response: signers, sessions, the pad itself (terminal 4)
	scripts/pad.sh flow
