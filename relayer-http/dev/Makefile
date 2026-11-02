# Development tooling for relayer-http against the local fhevm stack. Run from anywhere:
#   make -C relayer-http/dev <rule>       (see README.md)
# relayer-http runs in the stack (./fhevm-cli up) on :8080; `make run` runs it on the host on :18080.
ROOT    := $(abspath ..)
CLIPPY  := cargo clippy --workspace --all-targets -- -W clippy::perf -W clippy::suspicious -W clippy::style -D warnings

.PHONY: help fmt fmt-fix clippy test test-aws check build run public-decrypt user-decrypt

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
run: ## cargo run the relayer on the host (:18080) with the stack's config, logs in this terminal
	scripts/relayer-run.sh

# --- flows (e2e debug container) ----------------------------------------------------------------------------------
public-decrypt: ## deploy HTTPPublicDecrypt and POST /v4/public-decrypt (ATTEMPTS, RETRY_DELAY_MS, RELAYER_HTTP_URL)
	scripts/e2e.sh public-decrypt.ts
user-decrypt: ## deploy UserDecrypt, sign the unified request, POST /v4/user-decrypt (+ MIN_SHARES)
	scripts/e2e.sh user-decrypt.ts
