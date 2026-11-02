# Development tooling for relayer-http against the local fhevm stack. Run from anywhere:
#   make -C relayer-http/dev <rule>       (see README.md)
ROOT    := $(abspath ..)
# relayer-http ports, passed to the scripts; override on the command line (make RELAYER_HTTP_RUN_PORT=5002 run).
# RELAYER_HTTP_PORT: relayer-http in the stack (./fhevm-cli up), as in test-suite/fhevm/src/layout.ts.
# RELAYER_HTTP_RUN_PORT: relayer-http on the host (make run); its forwarders take the next ports.
export RELAYER_HTTP_PORT     ?= 3002
export RELAYER_HTTP_RUN_PORT ?= 4002
CLIPPY  := cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings

.PHONY: help fmt fmt-fix clippy test test-aws check build run down public-decrypt user-decrypt

help: ## this list
	@grep -E '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{ printf "  %-16s %s\n", $$1, $$2 }'

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
check: fmt clippy test ## fmt + clippy + test
build: ## cargo build -p relayer-http
	cd $(ROOT) && cargo build -p relayer-http

# --- local stack ------------------------------------------------------------------------------------------------
run: ## cargo run the relayer on the host (RELAYER_HTTP_RUN_PORT, default 4002) with the stack's config, logs in this terminal
	scripts/relayer-run.sh
down: ## remove the forwarders `make run` leaves running
	@ids="$$(docker ps -aq --filter name='^relayer-http-fwd-')"; [ -z "$$ids" ] || docker rm -f $$ids

# --- flows (e2e debug container) ----------------------------------------------------------------------------------
public-decrypt: ## deploy HTTPPublicDecrypt and POST /v4/public-decrypt (ATTEMPTS, RETRY_DELAY_MS, RELAYER_HTTP_URL)
	scripts/e2e.sh public-decrypt.ts
user-decrypt: ## deploy UserDecrypt, sign the unified request, POST /v4/user-decrypt (+ MIN_SHARES)
	scripts/e2e.sh user-decrypt.ts
