# Orchestration for the sdk workspace.
#
# This file owns the graph BETWEEN packages. Each package keeps owning the order WITHIN itself: its own
# `build` already encodes a sequence with real subtleties — v12's `build:templates` embeds a forge build
# that `generate:local-host-bytecode` then depends on — and re-deriving that here would duplicate
# knowledge that already has a correct home.
#
# What Make adds is the edge npm cannot express: `hardhat/v2/e2e`'s LINT consumes the plugin's BUILD
# output, and `npm run <script> --workspaces` can only sequence whole packages for one script name. That
# is why a repo-wide "lint everything, then build everything" is unsatisfiable, and why the phases and
# their prerequisites live here instead.
#
# Stamps under $(STAMPS) stand in for build outputs, so a package is rebuilt only when its own sources
# change. Delete a stamp (or run `make clean`) to force one.
#
#   make            list the targets
#   make build      everything, in dependency order
#   make lint       every package's lint, building only what a lint actually requires
#   make -j4 build  the same, parallel across independent branches
#
# `build:templates` is deliberately NOT called here: it is the `build:tarball` flow and assumes an
# already-built tree, so running it first on a clean checkout fails in generate:local-host-bytecode. Each
# package's plain `build` is the clean-safe entry point.
#
# Parallel-safe, though `build` is a linear chain so `-j` only pays off on `lint`.

SHELL := /usr/bin/env bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := help

STAMPS := .make

COMMON       := common
VENDORED     := common-vendored
V12          := host-contracts-cleartext/v12
V13          := host-contracts-cleartext/v13
PLUGIN       := hardhat/v2/plugin
E2E          := hardhat/v2/e2e
NPM_CLI      := fhevm-npm

# `npm run -w <name>` resolves from anywhere in the workspace, so recipes never cd.
W_COMMON   := @fhevm/sdk-common-dev
W_VENDORED := @fhevm/sdk-vendored-dev
W_V12      := @fhevm/host-contracts-cleartext-v12-dev
W_V13      := @fhevm/host-contracts-cleartext-v13-dev
W_PLUGIN   := @fhevm/hardhat-plugin-v2-dev
W_E2E      := @fhevm/hardhat-plugin-v2-e2e-dev

run = npm run --silent $(2) -w $(1)

# Sources that make a package stale. Build output and installed dependencies are pruned: including them
# would make every build dirty the moment it finished.
prune := -name node_modules -o -name _cjs -o -name _esm -o -name _types -o -name artifacts \
         -o -name cache -o -name out -o -name typechain-types -o -name dependencies -o -name .make
sources = $(shell find $(1) \( $(prune) \) -prune -o -type f \
            \( -name '*.ts' -o -name '*.sol' -o -name '*.json' -o -name '*.mjs' \) -print 2>/dev/null)

COMMON_SRC   := $(call sources,$(COMMON))
VENDORED_SRC := $(call sources,$(VENDORED))
V12_SRC      := $(call sources,$(V12))
V13_SRC      := $(call sources,$(V13))
PLUGIN_SRC   := $(call sources,$(PLUGIN))
E2E_SRC      := $(call sources,$(E2E))

########################################################################################################
# Aggregates
########################################################################################################

.PHONY: help graph build rebuild ci lint test check fmt fmt-check clean clean-stamps install

help: ## List the targets
	@grep -hE '^[a-zA-Z0-9_-]+:.*?## ' $(MAKEFILE_LIST) \
	  | awk -F':.*?## ' '{ printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2 }'

build: build-e2e ## Build every package, in dependency order

lint: lint-common lint-vendored lint-v12 lint-v13 lint-plugin lint-e2e ## Lint every package

test: test-v12 test-v13 test-plugin test-e2e ## Run every package's tests against a built tree

check: ## Workspace-wide gates that belong to no single package
	npm run check:foundry
	npm run check:vendored-sync
	npm run check:dep-versions
	npm run check:tsconfig-paths
	npm run check:pack-scripts

fmt: ## Rewrite formatting everywhere
	npm run prettier:write --workspaces --if-present
	npm run forge:fmt --workspaces --if-present

fmt-check: ## Verify formatting everywhere
	npm run prettier:check --workspaces --if-present
	npm run forge:fmt:check --workspaces --if-present

install: ## Install workspace and forge dependencies
	npm install
	npm run install:forge-dependencies

# Two sub-makes rather than `rebuild: clean build`: prerequisites of one target may run in any order
# under `-j`, which would race the clean against the build.
ci: ## Everything, in order: workspace gates, build, lint, test
	$(MAKE) check
	$(MAKE) build
	$(MAKE) lint
	$(MAKE) test

rebuild: ## Clean everything, then build from scratch
	$(MAKE) clean
	$(MAKE) build

clean: clean-stamps ## Remove every package's build output
	npm run clean --workspaces --if-present
	rm -rf ./tarballs

clean-stamps: ## Forget what Make believes is built
	rm -rf $(STAMPS)

$(STAMPS):
	@mkdir -p $(STAMPS)

# `make graph` answers "what would this build, and why", without running anything. It resolves against a
# throwaway stamp directory so the answer is the graph itself rather than whatever happens to be built.
# GNU Make 3.81 (what macOS ships) has no --trace, but --debug=b prints the same tree, deepest first.
graph: ## Print the dependency graph for TARGET (default: build)
	@tmp=$$(mktemp -d); \
	$(MAKE) --debug=b -n $(or $(TARGET),build) STAMPS=$$tmp 2>&1 \
	  | grep 'Must remake target' \
	  | sed -e "s/Must remake target .//" -e "s/.\.$$//" -e "s|$$tmp/|stamp:|" \
	  | awk '{ l[NR] = $$0 } END { for (i = NR; i > 0; i--) print l[i] }'; \
	rm -rf $$tmp

########################################################################################################
# Build graph
#
# v12 -> v13 -> plugin -> e2e. Each edge is a real consumption of the previous package's BUILD OUTPUT:
# v13 devDepends on v12, plugin/pkg imports @fhevm/host-contracts-cleartext (= v13/pkg), and e2e imports
# both @fhevm/hardhat-plugin (= plugin/pkg) and @fhevm/host-contracts-cleartext.
########################################################################################################

.PHONY: build-v12 build-v13 build-plugin build-e2e

build-v12: $(STAMPS)/v12.build ## Build host-contracts-cleartext/v12
build-v13: $(STAMPS)/v13.build ## Build host-contracts-cleartext/v13
build-plugin: $(STAMPS)/plugin.build ## Build the Hardhat v2 plugin
build-e2e: $(STAMPS)/e2e.build ## Build the Hardhat v2 e2e suite

$(STAMPS)/v12.build: $(V12_SRC) | $(STAMPS)
	@echo "==> build $(V12)"
	$(call run,$(W_V12),build)
	@touch $@

$(STAMPS)/v13.build: $(V13_SRC) $(STAMPS)/v12.build | $(STAMPS)
	@echo "==> build $(V13)"
	$(call run,$(W_V13),build)
	@touch $@

$(STAMPS)/plugin.build: $(PLUGIN_SRC) $(STAMPS)/v13.build | $(STAMPS)
	@echo "==> build $(PLUGIN)"
	$(call run,$(W_PLUGIN),build)
	@touch $@

# typechain is e2e's real prerequisite: both its build and its lint need the generated contract types,
# and generating them loads hardhat.config.ts, which requires the built plugin.
$(STAMPS)/e2e.typechain: $(E2E_SRC) $(STAMPS)/plugin.build $(STAMPS)/v13.build | $(STAMPS)
	@echo "==> typechain $(E2E)"
	$(call run,$(W_E2E),typechain)
	@touch $@

$(STAMPS)/e2e.build: $(STAMPS)/e2e.typechain | $(STAMPS)
	@echo "==> build $(E2E)"
	$(call run,$(W_E2E),build)
	@touch $@

########################################################################################################
# Lint graph
#
# Type-aware linting reads its dependencies' DECLARATIONS, so a lint edge points at a build, not a lint.
# This is the part npm cannot express, and the reason `npm run lint --workspaces` fails on a clean tree.
########################################################################################################

.PHONY: lint-common lint-vendored lint-v12 lint-v13 lint-plugin lint-e2e lint-npm-cli

lint-common: ## Lint common
	$(call run,$(W_COMMON),lint)

lint-vendored: ## Lint common-vendored
	$(call run,$(W_VENDORED),lint)

lint-v12: $(STAMPS)/v12.build ## Lint v12 (needs its own generated sources)
	$(call run,$(W_V12),lint)

lint-v13: $(STAMPS)/v13.build ## Lint v13
	$(call run,$(W_V13),lint)

lint-plugin: $(STAMPS)/v13.build ## Lint the plugin (its types resolve through v13/pkg)
	$(call run,$(W_PLUGIN),lint)

lint-e2e: $(STAMPS)/e2e.typechain ## Lint e2e (needs typechain, which needs the built plugin)
	$(call run,$(W_E2E),lint)

lint-npm-cli: ## Typecheck and test the fhevm-npm CLI
	npm --prefix $(NPM_CLI) run typecheck
	npm --prefix $(NPM_CLI) test

########################################################################################################
# Tests
#
# Tests assume a BUILT tree — `make build test`, not `make test` alone. They declare no build
# prerequisite on purpose: re-running one test should never trigger a 90s rebuild, and a test target that
# silently rebuilds hides what it costs. `make ci` is the one-liner that orders the whole thing.
########################################################################################################

.PHONY: test-v12 test-v13 test-plugin test-e2e test-e2e-anvil test-consumer

test-v12: ## v12 unit + forge + harness tests
	$(call run,$(W_V12),test)

test-v13: ## v13 unit + forge + harness tests
	$(call run,$(W_V13),test)

test-plugin: ## Plugin tests
	$(call run,$(W_PLUGIN),test)

# HARDHAT_NETWORK=hardhat — the in-process network, so this needs nothing running. The variants below
# do: `anvil` a local node, `sepolia`/`devnet` a funded remote account.
test-e2e: ## e2e tests on the in-process Hardhat network
	$(call run,$(W_E2E),test)

test-e2e-anvil: ## e2e tests against a local anvil (must already be running)
	$(call run,$(W_E2E),test:anvil)

# Separate tier: packs and installs each payload into a temp project and runs its suite. Minutes, not
# seconds, so it is deliberately not part of `make test`.
test-consumer: ## Install each published payload as a real consumer
	npm run test:consumer --workspaces --if-present

########################################################################################################
# Vendored sources
########################################################################################################

.PHONY: sync-vendored check-vendored

sync-vendored: ## Write every vendored destination from its source of truth
	npm run sync:vendored

check-vendored: ## Verify vendored destinations and their pinned provenance
	npm run check:vendored
