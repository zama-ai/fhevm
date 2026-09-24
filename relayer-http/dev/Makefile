# Development tooling for relayer-http against the local fhevm stack. Run from anywhere:
#   make -C relayer-http/dev <rule>       (see README.md)
# KMS defaults to the number of connector endpoints running in the stack; override with KMS=<n>.
ROOT    := $(abspath ..)
REPO    := $(abspath ../..)
KMS     ?= $(shell docker ps --format '{{.Names}}' 2>/dev/null | grep -cE '^kms-connector(-[0-9]+)?-endpoint$$')
T       ?= $(shell grep -s '^KMS_THRESHOLD=' $(REPO)/.fhevm/runtime/env/test-suite.env | cut -d= -f2)
CONFIG  ?= $(ROOT)/dev/config/kms-$(KMS).yaml
API_KEY ?= fhevm-e2e-kms-connector-api-key
export API_KEY
CLIPPY  := cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings

.PHONY: help fmt fmt-fix clippy test check build up down status run config public-decrypt user-decrypt

help: ## this list
	@grep -E '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{ printf "  %-16s %s\n", $$1, $$2 }'
	@echo "  detected: KMS=$(KMS) T=$(T) config=$(CONFIG)"

# --- code -------------------------------------------------------------------------------------------------------
fmt: ## cargo fmt -- --check (not --all: it would follow the kms-connector-api path dependency into generated bindings)
	cd $(ROOT) && cargo fmt -- --check
fmt-fix: ## cargo fmt
	cd $(ROOT) && cargo fmt
clippy: ## the clippy line every change must pass
	cd $(ROOT) && $(CLIPPY)
test: ## cargo test --workspace
	cd $(ROOT) && cargo test --workspace
check: fmt clippy test ## fmt + clippy + test
build: ## cargo build -p relayer-http
	cd $(ROOT) && cargo build -p relayer-http

# --- local stack ------------------------------------------------------------------------------------------------
up: ## proxies + forwarders for the KMS connectors of the running stack, then the smoke (KMS=<n>, TAG=<image tag>)
	@[ "$(KMS)" -gt 0 ] || { echo "no kms-connector endpoint container: start the stack first (./fhevm-cli up)"; exit 1; }
	scripts/proxies-up.sh $(KMS) $(TAG)
down: ## remove the proxies and forwarders
	scripts/proxies-down.sh
status: ## smoke through forwarder -> proxy -> endpoint for every party
	scripts/proxies-status.sh $(KMS)
run: ## cargo run the relayer with the config of the detected topology, logs in this terminal
	@echo "KMS=$(KMS) config=$(CONFIG)"
	scripts/relayer-run.sh $(CONFIG)
config: ## write dev/config/kms-$(KMS).yaml for KMS connectors and threshold T (make config KMS=7 T=2)
	@[ -n "$(T)" ] || { echo "T (MPC threshold) unknown: make config KMS=<n> T=<t>"; exit 1; }
	scripts/gen-config.sh $(KMS) $(T)

# --- flows (run inside the e2e debug container, against the relayer on the host) ----------------------------------
public-decrypt: ## deploy HTTPPublicDecrypt and POST /v4/exp/public-decrypt
	scripts/e2e.sh public-decrypt.ts
user-decrypt: ## deploy UserDecrypt, sign the unified request and POST /v4/exp/user-decrypt
	scripts/e2e.sh user-decrypt.ts
