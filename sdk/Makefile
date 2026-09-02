# Orchestration for the sdk workspace.
#
# This file owns the graph BETWEEN packages, and speaks to each package ONLY through its public verbs:
# clean, clean:generated, generate, fmt, fmt:check, lint, compile, check, test (plus test:consumer
# where it applies) — and `build`, the everyday sweep: fmt:check → lint → compile. Everything below a
# verb — phases, leaf generators, per-tool scripts — is the package's private business; each package owns the order WITHIN its own verbs, and re-deriving that here would
# duplicate knowledge with a correct home. The named exceptions are workspace-level, not package-level:
# the fhevm-npm formatting (a non-member `-w` cannot reach) and the `test:anvil` operator target
# (a manual entry point, not lifecycle orchestration).
#
# What Make adds is the edge npm cannot express: `hardhat/v2/e2e`'s LINT consumes the plugin's BUILD
# output, and `npm run <script> --workspaces` can only sequence whole packages for one script name. That
# is why a repo-wide "lint everything, then build everything" is unsatisfiable, and why the phases and
# their prerequisites live here instead.
#
# There are deliberately NO build stamps. Every `compile-*` target is phony and runs whenever it is
# asked for. A stamp needs an exhaustive list of that package's inputs, and no such list stays right:
# the old one matched four extensions in seven directories, so editing `foundry.toml` or
# `remappings.txt`, or deleting a source, left the stamp fresh and the output stale. An unnecessary
# compile is slow; a skipped one validates stale output and reports a false green. Make still runs each
# phony target at most once per invocation, so `make compile` compiles each package exactly once. The
# tools keep their own incremental caches — Make simply always asks them.
#
#   make              list the targets
#   make compile      every artifact, in dependency order
#   make build        the same, gated: fmt-check, lint, then compile
#   make lint         every package's lint, compiling whatever a lint actually requires first
#   make -j4 compile  the same, parallel across independent branches
#
# Generation is NOT part of compiling. Generated output here is committed — pkg/ts/index.ts is the
# package's public surface, compiled by tsc and imported by consumers — which makes it a compile
# SOURCE, and a compile must not rewrite its own inputs.
#
# Three verbs, one job each, exactly as formatting already works here:
#   make generate   writes tracked files, deliberately, and you commit the result
#   make compile    consumes them, writing only into output directories
#   check-generated deletes every generated file, regenerates, and requires a spotless tree — the one
#                   sequence that also catches a generator that silently STOPPED emitting a file.
#
# Parallel-safe, though `compile` is a linear chain so `-j` only pays off on `lint`.

# The flags live in SHELL, not in .SHELLFLAGS: .SHELLFLAGS is a GNU Make 4.0 feature and macOS ships
# 3.81, which ignores it silently — so `set -o pipefail` was never actually in effect. Verified both ways
# on 3.81: with .SHELLFLAGS a failing pipe let the recipe continue; with the flags on SHELL it aborts.
SHELL := /usr/bin/env bash -eu -o pipefail
.SHELLFLAGS := -c
.DEFAULT_GOAL := help

DIR_COMMON           := common
DIR_COMMON_VENDORED  := common-vendored
DIR_V12              := host-contracts-cleartext/v12
DIR_V13              := host-contracts-cleartext/v13
DIR_HH_V2_PLUGIN     := hardhat/v2/plugin
DIR_HH_V2_E2E        := hardhat/v2/e2e
DIR_HH_V2_TEMPLATE   := hardhat/v2/fhevm-hardhat-template
DIR_FHEVM_NPM        := fhevm-npm
FHEVM_NPM_CLI        := ./fhevm-npm-cli

# Global flags spliced in BEFORE the subcommand, for any target that calls the CLI. Mainly verbosity,
# which is how you see per-package detail and the timings a check reports:
#
#   make check-pre              FHEVM_NPM_ARGS=-v     # ✅/❌ per package
#   make check-vendored-origin  FHEVM_NPM_ARGS=-vv    # the above plus ⏱️  timings
#
# Repeatable up to -vvvv. Any global flag the CLI accepts works here, not just verbosity.
FHEVM_NPM_ARGS       ?=

# `npm run -w <name>` resolves from anywhere in the workspace, so recipes never cd.
W_COMMON             := @fhevm/sdk-common-dev
W_VENDORED           := @fhevm/sdk-vendored-dev
W_V12                := @fhevm/host-contracts-cleartext-v12-dev
W_V13                := @fhevm/host-contracts-cleartext-v13-dev
W_HH_V2_PLUGIN       := @fhevm/hardhat-plugin-v2-dev
W_HH_V2_E2E          := @fhevm/hardhat-plugin-v2-e2e-dev
W_HH_V2_TEMPLATE     := @fhevm/hardhat-template-v2-dev
W_HH_V2_TEMPLATE_PKG := fhevm-hardhat-template-v2

run = npm run --silent $(2) -w $(1)

# `fhevm-npm` is deliberately NOT a workspace member, so `-w` cannot reach it. It still needs
# formatting: it was once outside `fmt` entirely, which is how unformatted files reached ci.
# (`scripts/` holds only .sh by policy — nothing there for prettier.)
run-prefix = npm run --silent $(2) --prefix ./$(1)

run-fhevm-npm = $(FHEVM_NPM_CLI) $(FHEVM_NPM_ARGS) $(1)

########################################################################################################
# Aggregates
########################################################################################################

.PHONY: help graph build compile rebuild ci ci-from-scratch pristine lint test check check-pre generate fmt fmt-check clean clean-generated install

help: ## List the targets
	@grep -hE '^[a-zA-Z0-9_-]+:.*?## ' $(MAKEFILE_LIST) \
	  | awk -F':.*?## ' '{ \
	      if ($$1 ~ /^(build|check|check-pre|ci|ci-from-scratch|clean|compile|graph|help|install|lint|pristine|rebuild|test)$$/) group = 0; \
	      else if ($$1 ~ /^compile-/) group = 1; \
	      else if ($$1 ~ /^check-/) group = 2; \
	      else if ($$1 ~ /^fmt($$|-)/) group = 3; \
	      else if ($$1 ~ /^lint-/) group = 4; \
	      else if ($$1 ~ /^test-/) group = 5; \
	      else group = 6; \
	      printf "%d\t%s\t%s\n", group, $$1, $$2; \
	    }' \
	  | sort -k1,1n -k2,2 \
	  | awk -F'\t' '{ \
	      if (NR > 1 && $$1 != group) print ""; \
	      group = $$1; \
	      printf "  \033[36m%-22s\033[0m %s\n", $$2, $$3; \
	    }'

compile: compile-hh-v2-e2e compile-hh-v2-template ## Compile every package, in dependency order

# The everyday sweep, in lifecycle order. ONE sub-make with three goals, deliberately: goals run left
# to right in a single invocation, so the compiles `lint` triggers are not re-run by `compile`.
# Overloading `build` fails in the safe direction — muscle-memory `make build` still produces every
# artifact, it just gates them behind formatting and lint first.
build: ## fmt-check, then lint, then compile — everything, gated
	$(MAKE) fmt-check lint compile

lint: lint-shared lint-cleartext lint-hh-v2 lint-npm-cli ## Lint every package

test: test-cleartext-v12 test-cleartext-v13 test-hh-v2-plugin test-hh-v2-template test-hh-v2-e2e ## Run package tests only; this is not the full validation workflow (use: 'make ci' instead)

check: check-npm-cli ## Run pre-build checks, build, then post-build checks

check-pre: check-npm-cli-pre-build ## Run all checks that do not require build output

# Vendored content settles FIRST — and in two steps: common-vendored's own `generate` renders
# sdk/cleartext-config.json into common-vendored/src, THEN sync fans the sources out into pkg/src and
# pkg/ts, which the generators read and forge compiles. Each package's `generate` then owns its internal
# straddle (pre → forge → post).
generate: ## Write every generated file (then commit the result)
	$(call run,$(W_VENDORED),generate)
	$(MAKE) sync-common-vendored
	$(call run,$(W_V12),generate)
	$(call run,$(W_V13),generate)

# Each package's `fmt` owns what formatting means there (prettier, plus forge fmt where it has
# Solidity). fhevm-npm is a non-member `-w` cannot reach — the one named workspace-level exception.
# scripts/ holds only .sh by policy, so nothing there is prettier's business.
fmt: ## Rewrite formatting everywhere
	$(call run,$(W_COMMON),fmt)
	$(call run,$(W_VENDORED),fmt)
	$(call run,$(W_V12),fmt)
	$(call run,$(W_V13),fmt)
	$(call run,$(W_HH_V2_PLUGIN),fmt)
	$(call run,$(W_HH_V2_TEMPLATE),fmt)
	$(call run,$(W_HH_V2_E2E),fmt)
	$(call run-prefix,$(DIR_FHEVM_NPM),prettier:write)

fmt-check: ## Verify formatting everywhere
	$(call run,$(W_COMMON),fmt:check)
	$(call run,$(W_VENDORED),fmt:check)
	$(call run,$(W_V12),fmt:check)
	$(call run,$(W_V13),fmt:check)
	$(call run,$(W_HH_V2_PLUGIN),fmt:check)
	$(call run,$(W_HH_V2_TEMPLATE),fmt:check)
	$(call run,$(W_HH_V2_E2E),fmt:check)
	$(call run-prefix,$(DIR_FHEVM_NPM),prettier:check)

# fhevm-npm is deliberately NOT a workspace member, so the root `npm install` never reaches it — and
# every later target needs it, because the fhevm-npm CLI is how forge dependencies get installed.
install: ## Install workspace, fhevm-npm, and forge dependencies
	npm install
	npm --prefix $(DIR_FHEVM_NPM) install
	$(call run-fhevm-npm,install-forge-dependencies)

# Deliberately NOT `git clean`: pattern-based deletion cannot distinguish a build output from a
# developer's .vscode or .env, and gitignored files are invisible to the spotless guard — an editor
# setup or a secret must never be the price of a from-scratch test. Everything here is explicit and
# regenerable: build outputs via each package's own clean, node_modules via the audited script,
# forge dependency trees asked of `forge config --json`.
#
# The FULL list is shown first and confirmed ONCE, before anything at all is deleted; only after the
# yes do the sub-deleters run with --force (their own questions would be redundant — this gate IS
# the question). A non-interactive stdin aborts unless FORCE=1 is passed explicitly.
# Ordering matters twice: `clean` runs npm scripts, so it needs node_modules; the forge cleaner runs
# through the fhevm-npm CLI, so it needs fhevm-npm's.
pristine: ## Reset sdk/ to a fresh-clone state: shows everything first, asks once, then deletes
	@echo "🧹 make pristine will remove ALL of the following:"
	@echo ""
	@echo "── Forge dependency trees (from forge config --json) ─────────────────────────"
	$(call run-fhevm-npm,clean-forge-dependencies --dry-run)
	@echo ""
	@echo "── node_modules ───────────────────────────────────────────────────────────────"
	./scripts/clean-node-modules.sh --dry-run
	@echo ""
	@echo "── Build outputs ──────────────────────────────────────────────────────────────"
	@echo "   Everything 'make clean' removes: each package's emit dirs (pkg/ts/_*, pkg/_*),"
	@echo "   forge cache/out/broadcast, hardhat artifacts, tsbuildinfo files, ./tarballs."
	@echo ""
	@if [ "$(FORCE)" != "1" ]; then \
	  printf 'Remove ALL of the above? [Y/n] '; \
	  read -r reply; \
	  case "$$reply" in [nN]|[nN][oO]) echo "Aborted. Nothing was removed."; exit 1;; esac; \
	fi
	$(MAKE) clean
	$(call run-fhevm-npm,clean-forge-dependencies --force)
	./scripts/clean-node-modules.sh --force

# The strongest proof this workspace has: a brand-new machine could clone, install, and pass every
# gate. Sub-makes for the same `-j` ordering reason `ci` uses them.
ci-from-scratch: ## Prove a fresh clone works: pristine wipe, install, then every ci gate
	$(MAKE) pristine
	$(MAKE) install
	$(MAKE) ci

# Two sub-makes rather than `rebuild: clean build`: prerequisites of one target may run in any order
# under `-j`, which would race the clean against the build.
# The maximum run: from a clean tree, every gate this workspace has, cheapest first so a failure costs
# the least time. `clean` is deliberate — stamps make `build` skippable, and validating a stale payload
# with publint/attw would be worse than not validating it.
#
# Excluded on purpose: `test-hh-v2-e2e-anvil`, which needs an anvil node already running.
# The spotless check runs FIRST, before anything at all: check-generated needs it anyway, and
# discovering it only there would have already thrown away every build output for nothing.
ci: ## Run EVERY gate from a clean tree: formatting, checks, compile, lint, tests, consumer rehearsal
	$(call require-spotless-worktree)
	$(MAKE) clean
	$(MAKE) check-generated
	$(MAKE) check-pre
	$(MAKE) check-vendored-origin
	$(MAKE) build
	$(MAKE) check-post
	$(MAKE) test
	$(MAKE) test-consumer-ci

rebuild: ## Clean everything, then compile from scratch (no gates — use 'build' for the gated form)
	$(MAKE) clean
	$(MAKE) compile

# Reverse dependency order — dependents before their dependencies — so nothing is ever cleaned while
# something downstream of it is still expected to be usable. The template goes first because it consumes
# the plugin, which consumes v13.
#
# Note the template's own `clean` deliberately does NOT delegate to `pkg`'s: upstream's version ends by
# running `npm run typechain`, and a clean that regenerates would need the plugin it just helped remove.
# The parent removes pkg's output directly instead.
clean: ## Remove every package's build output
	$(call run,$(W_HH_V2_TEMPLATE),clean)
	$(call run,$(W_HH_V2_E2E),clean)
	$(call run,$(W_HH_V2_PLUGIN),clean)
	$(call run,$(W_V13),clean)
	$(call run,$(W_V12),clean)
	$(call run,$(W_VENDORED),clean)
	$(call run,$(W_COMMON),clean)
	rm -rf ./tarballs

# Deliberately NOT part of `clean`: this deletes committed, shipped payload, so a `clean` that did it
# would leave `make clean && make build` emitting an incomplete package.
#
# It exists for the one thing `check-generated` cannot prove on its own — that the generators still emit
# EVERY file. A generator that silently stops writing one leaves the committed copy untouched, so nothing
# looks dirty and the gate passes. Delete first and the missing file shows up as a deletion.
#
# Each package owns its own list, as `clean:generated`, because that list belongs beside the generators
# that write it. The one guard that has to live here is the safety one: a fully clean `git status` means
# everything about to be deleted is committed, so any mistake — including in a list this file has never
# seen — is undone by `git checkout -- .`. That is why it tests `git status`, not `git diff`: `git diff`
# ignores untracked files, and an untracked file deleted is gone for good.
# The command that ran this will (partway through) DELETE committed generated files and regenerate
# them — only recoverable from a spotless tree. Shared by `ci` (which fails fast, before anything at
# all has run) and `clean-generated` (the step that actually deletes).
define require-spotless-worktree
@[ -z "$$(git status --porcelain)" ] || { \
  echo ""; \
  echo "🛑 ══════════════════════════════════════════════════════════════════════════"; \
  echo "🛑  STOPPED — your git working tree has uncommitted changes"; \
  echo "🛑 ══════════════════════════════════════════════════════════════════════════"; \
  echo ""; \
  echo "   The command you ran deletes files wholesale — committed generated files"; \
  echo "   it will regenerate, or everything a fresh clone lacks. That is only safe"; \
  echo "   on a spotless tree: anything committed is undone with  git checkout -- ."; \
  echo ""; \
  echo "   ⚠️  With uncommitted changes, that same recovery would wipe YOUR work too."; \
  echo "      Nothing has been deleted. Nothing has been touched."; \
  echo ""; \
  echo "   👉 Pick one, then re-run your command:"; \
  echo ""; \
  echo "      1️⃣  Commit your work:       git add -A && git commit"; \
  echo "      2️⃣  Or set it aside:        git stash --include-untracked"; \
  echo "          (get it back later:     git stash pop)"; \
  echo ""; \
  echo "   🔍 What git considers uncommitted right now:  git status"; \
  echo ""; \
  exit 1; }
endef

clean-generated: ## Delete every regenerable file, to prove `make generate` reproduces them from nothing
	$(call require-spotless-worktree)
	$(call run,$(W_VENDORED),clean:generated)
	$(call run,$(W_V12),clean:generated)
	$(call run,$(W_V13),clean:generated)
	@echo "Deleted. Run 'make generate' - a spotless 'git status' proves the generators reproduce everything."

# `make graph` answers "what would this build, and why", without running anything.
# GNU Make 3.81 (what macOS ships) has no --trace, but --debug=b prints the same tree, deepest first.
graph: ## Print the dependency graph for TARGET (default: compile)
	@$(MAKE) --debug=b -n $(or $(TARGET),compile) 2>&1 \
	  | grep 'Must remake target' \
	  | sed -e "s/Must remake target .//" -e "s/.\.$$//" \
	  | awk '{ l[NR] = $$0 } END { for (i = NR; i > 0; i--) print l[i] }'

########################################################################################################
# Build graph
#
# v12 -> v13 -> plugin -> e2e. Each edge is a real consumption of the previous package's BUILD OUTPUT:
# v13 devDepends on v12, plugin/pkg imports @fhevm/host-contracts-cleartext (= v13/pkg), and e2e imports
# both @fhevm/hardhat-plugin (= plugin/pkg) and @fhevm/host-contracts-cleartext.
########################################################################################################

.PHONY: compile-package compile-cleartext-v12 compile-cleartext-v13 compile-hh-v2-plugin compile-hh-v2-template
.PHONY: compile-hh-v2-e2e

# Public bridge for tools that discover a package by manifest path while keeping this file authoritative
# for the actual build graph. Unknown packages fail instead of silently bypassing Make with `npm run build`.
compile-package: ## Compile PACKAGE and its prerequisites
	+@case "$(PACKAGE)" in \
	  "./$(DIR_V12)") target=compile-cleartext-v12 ;; \
	  "./$(DIR_V13)") target=compile-cleartext-v13 ;; \
	  "./$(DIR_HH_V2_PLUGIN)") target=compile-hh-v2-plugin ;; \
	  "./$(DIR_HH_V2_TEMPLATE)") target=compile-hh-v2-template ;; \
	  "./$(DIR_HH_V2_E2E)") target=compile-hh-v2-e2e ;; \
	  *) echo "Unsupported build package '$(PACKAGE)'" >&2; exit 2 ;; \
	esac; \
	$(MAKE) --no-print-directory "$$target"

compile-cleartext-v12: check-npm-cli-pre-build ## Compile host-contracts-cleartext/v12
	@echo "==> compile $(DIR_V12)"
	$(call run,$(W_V12),compile)

compile-cleartext-v13: compile-cleartext-v12 ## Compile host-contracts-cleartext/v13
	@echo "==> compile $(DIR_V13)"
	$(call run,$(W_V13),compile)

compile-hh-v2-plugin: compile-cleartext-v13 ## Compile the Hardhat v2 plugin
	@echo "==> compile $(DIR_HH_V2_PLUGIN)"
	$(call run,$(W_HH_V2_PLUGIN),compile)

compile-hh-v2-template: compile-hh-v2-plugin ## Compile the mirrored Hardhat v2 template
	@echo "==> compile $(DIR_HH_V2_TEMPLATE)"
	$(call run,$(W_HH_V2_TEMPLATE),compile)

# e2e's `compile` (hardhat compile) generates its own typechain types; its `lint` self-provides them
# the same way. Both need the compiled plugin, because loading hardhat.config.ts requires it.
compile-hh-v2-e2e: compile-hh-v2-plugin ## Compile the Hardhat v2 e2e suite
	@echo "==> compile $(DIR_HH_V2_E2E)"
	$(call run,$(W_HH_V2_E2E),compile)

########################################################################################################
# Check graph
########################################################################################################

.PHONY: check-npm-cli check-npm-cli-pre-build check-npm-cli-post-build check-mirror

check-npm-cli: check-npm-cli-post-build # Internal: run every fhevm-npm check in lifecycle order

# Deliberately PHONY, so it re-runs every time something asks for it — four times in a `make ci`, and
# that is the correct trade. Stamping it needs an exhaustive list of what these checks read, and that
# list cannot be right: check-tsconfig-paths reads every tsconfig.json, check-lint-policy reads the
# eslint configs, and neither is package metadata. A stamp whose inputs are incomplete SKIPS a check that
# should have run. Re-running a cheap check costs seconds; missing one costs a broken release.
check-npm-cli-pre-build: # Internal: run fhevm-npm checks that do not require generated output
	$(call run-fhevm-npm,check-manifest-coverage)
	$(call run-fhevm-npm,check-names)
	$(call run-fhevm-npm,check-workspaces)
	$(call run-fhevm-npm,check-ownership)
	$(call run-fhevm-npm,check-package-json)
	$(call run-fhevm-npm,check-dependencies)
	$(call run-fhevm-npm,check-scripts)
	$(call run-fhevm-npm,check-tsc-mode)
	$(call run-fhevm-npm,check-lockfiles)
	$(call run-fhevm-npm,check-foundry)
	$(call run-fhevm-npm,check-lint-policy)
	$(call run-fhevm-npm,check-cleartext-config)
	$(call run-fhevm-npm,sync-vendored --check)

# Not wired into `check-pre` or `ci`: the mirror spec is not implemented yet. Running it today reports 7
# violations, but they are the spec's gaps rather than real drift — it has not been taught about the
# workspace adaptations (renamed eslint/prettier configs, dropped .vscode/). The target exists so the
# work stays visible and addressable; swap the echo for the real check once the spec lands.
check-mirror: ## Verify the hardhat template still mirrors upstream byte-for-byte (NOT YET IMPLEMENTED)
	@echo "⚠️  Not Yet Implemented Warning: check-mirror is a placeholder."
	@echo "⚠️  The mirror spec does not yet describe this workspace's adaptations, so the real check"
	@echo "⚠️  ($(DIR_HH_V2_TEMPLATE) -> npm run check:mirror) still reports false violations."

check-npm-cli-post-build: compile-hh-v2-e2e compile-hh-v2-template # Internal: check generated paths
	$(call run-fhevm-npm,check-package-json-paths)
	# Both of these validate literal paths, and some of those paths are build output — the template's
	# gitignored `types/` among them — so neither can run before the build.
	$(call run-fhevm-npm,check-tsconfig-paths)

########################################################################################################
# Post-build checks
#
# Each package's `check` validates its own deliverables (publint/attw, contract sizes, configs, vendored
# provenance). Its prerequisite is that `generate` and `build` have RUN in this worktree: forge's ./out
# comes from generate's internal compile, the tsc payload from build. The build deps below provide the
# payloads transitively; ./out is a documented prerequisite — in `ci` the gate's regeneration provides
# it, and a worktree where it never existed fails loudly (contract-sizes throws), never silently.
########################################################################################################

.PHONY: check-post

check-post: check-npm-cli-post-build ## Checks that require a generated and built tree
	$(call run,$(W_V12),check)
	$(call run,$(W_V13),check)
	$(call run,$(W_HH_V2_PLUGIN),check)

########################################################################################################
# Lint graph
#
# Type-aware linting reads its dependencies' DECLARATIONS, so a lint edge points at a build, not a lint.
# This is the part npm cannot express, and the reason `npm run lint --workspaces` fails on a clean tree.
########################################################################################################

.PHONY: lint-shared lint-cleartext lint-hh-v2
.PHONY: lint-common lint-common-vendored lint-cleartext-v12 lint-cleartext-v13
.PHONY: lint-hh-v2-plugin lint-hh-v2-template lint-hh-v2-e2e lint-npm-cli

lint-shared: lint-common lint-common-vendored

lint-cleartext: lint-cleartext-v12 lint-cleartext-v13

lint-hh-v2: lint-hh-v2-plugin lint-hh-v2-template lint-hh-v2-e2e

lint-common: ## Lint common
	$(call run,$(W_COMMON),lint)

lint-common-vendored: ## Lint common-vendored
	$(call run,$(W_VENDORED),lint)

lint-cleartext-v12: compile-cleartext-v12 ## Lint cleartext v12 (needs its own generated sources)
	$(call run,$(W_V12),lint)

lint-cleartext-v13: compile-cleartext-v13 ## Lint cleartext v13
	$(call run,$(W_V13),lint)

lint-hh-v2-plugin: compile-cleartext-v13 ## Lint the Hardhat v2 plugin (its types resolve through v13/pkg)
	$(call run,$(W_HH_V2_PLUGIN),lint)

lint-hh-v2-template: compile-hh-v2-plugin ## Lint the mirrored Hardhat v2 template against the built plugin
	$(call run,$(W_HH_V2_TEMPLATE),lint)

lint-hh-v2-e2e: compile-hh-v2-plugin ## Lint Hardhat v2 e2e (self-provides typechain; needs the built plugin)
	$(call run,$(W_HH_V2_E2E),lint)

lint-npm-cli: ## Typecheck and test the fhevm-npm CLI
	npm --prefix $(DIR_FHEVM_NPM) run typecheck
	npm --prefix $(DIR_FHEVM_NPM) test

########################################################################################################
# Tests
#
# Tests assume a BUILT tree — `make build test`, not `make test` alone. They declare no build
# prerequisite on purpose: re-running one test should never trigger a 90s rebuild, and a test target that
# silently rebuilds hides what it costs. `make ci` is the one-liner that orders the whole thing.
########################################################################################################

.PHONY: test-cleartext-v12 test-cleartext-v13 test-hh-v2-plugin test-hh-v2-template
.PHONY: test-hh-v2-e2e test-hh-v2-e2e-anvil test-consumer test-consumer-ci

test-cleartext-v12: compile-cleartext-v12 ## Cleartext v12 unit + forge + harness tests (self-provides ./out)
	$(call run,$(W_V12),test)

test-cleartext-v13: compile-cleartext-v13 ## Cleartext v13 unit + forge + harness tests (self-provides ./out)
	$(call run,$(W_V13),test)

test-hh-v2-plugin: compile-hh-v2-plugin ## Hardhat v2 plugin tests
	$(call run,$(W_HH_V2_PLUGIN),test)

test-hh-v2-template: compile-hh-v2-template ## Mirrored Hardhat v2 template tests
	$(call run,$(W_HH_V2_TEMPLATE_PKG),test)

# HARDHAT_NETWORK=hardhat — the in-process network, so this needs nothing running. The variants below
# do: `anvil` a local node, `sepolia`/`devnet` a funded remote account.
test-hh-v2-e2e: compile-hh-v2-e2e ## Hardhat v2 e2e tests on the in-process network
	$(call run,$(W_HH_V2_E2E),test)

test-hh-v2-e2e-anvil: compile-hh-v2-e2e ## Hardhat v2 e2e tests against a local anvil (must already be running)
	$(call run,$(W_HH_V2_E2E),test:anvil)

# Separate tier: packs and installs each payload into a temp project and runs its suite. Minutes, not
# seconds, so it is deliberately not part of `make test`.
test-consumer: compile-hh-v2-template ## Install each published payload as a real consumer
	$(call run-fhevm-npm,test-consumer ./host-contracts-cleartext/v12 --build-linked-dependencies --run)
	$(call run-fhevm-npm,test-consumer ./host-contracts-cleartext/v13 --build-linked-dependencies --run)
	$(call run-fhevm-npm,test-consumer ./hardhat/v2/fhevm-hardhat-template/pkg --build-linked-dependencies --run)
	$(call run,$(W_HH_V2_PLUGIN),test:consumer)

test-consumer-ci: compile-hh-v2-template ## Run every consumer with its committed lockfile
	$(call run-fhevm-npm,test-consumer ./host-contracts-cleartext/v12 --build-linked-dependencies --run --ci)
	$(call run-fhevm-npm,test-consumer ./host-contracts-cleartext/v13 --build-linked-dependencies --run --ci)
	$(call run-fhevm-npm,test-consumer ./hardhat/v2/fhevm-hardhat-template/pkg --build-linked-dependencies --run --ci)
	$(call run,$(W_HH_V2_PLUGIN),test:consumer)

########################################################################################################
# Generated sources
#
# Generation is its own flow, deliberately outside `build` and outside `test`: a build that rewrites its
# own tracked inputs can never settle, and stamps derived from those inputs go stale the moment it runs.
# So nothing here is a prerequisite of a build — you run `make generate` and commit the result, and ci
# only ever *checks* that the committed output is what the generators produce.
########################################################################################################

.PHONY: check-generated sync-common-vendored check-vendored-origin

# THE regeneration gate: delete every generated file, regenerate everything, and require the worktree to
# come back spotless. Deleting first is what regenerate-and-diff alone cannot do — a generator that
# silently STOPS emitting a file leaves the committed copy untouched and nothing looks dirty; after the
# delete, the missing file shows up as a deletion.
#
# There is no list of generated paths, deliberately: each package owns its own as `clean:generated`, and
# the final `git status --porcelain` catches drift wherever it lands — modified, deleted AND untracked
# (`git diff` would miss a generator that starts emitting a NEW file).
#
# The cost, accepted knowingly: this REQUIRES a spotless worktree (the guard in `clean-generated`
# enforces it), because deleting committed files is only recoverable from a clean tree. The old gate
# tolerated a dirty tree via snapshots but could not see dropped emitters.
check-generated: ## Delete every generated file, regenerate, and fail unless the tree returns spotless
	$(MAKE) clean-generated
	$(MAKE) generate
	@dirty="$$(git status --porcelain)"; \
	  if [ -n "$$dirty" ]; then \
	    echo ""; \
	    echo "🛑 ══════════════════════════════════════════════════════════════════════════"; \
	    echo "🛑  FAILED — the committed generated files do not match what the generators"; \
	    echo "🛑  actually produce"; \
	    echo "🛑 ══════════════════════════════════════════════════════════════════════════"; \
	    echo ""; \
	    echo "   Every generated file was deleted and regenerated from its sources."; \
	    echo "   If everything were in sync, git would now see a spotless tree."; \
	    echo "   Instead, these files differ from what is committed:"; \
	    echo ""; \
	    printf '%s\n' "$$dirty" | sed 's/^/      /'; \
	    echo ""; \
	    echo "   This usually means someone changed a generator INPUT (a contract in"; \
	    echo "   pkg/src, an export.manifest.json, a config, a vendored pin) without"; \
	    echo "   committing the regenerated output — or edited a generated file by hand."; \
	    echo ""; \
	    echo "   👉 How to fix:"; \
	    echo ""; \
	    echo "      1️⃣  Look at what changed:              git diff"; \
	    echo "      2️⃣  If the changes look right, keep:   git add -A && git commit"; \
	    echo "      3️⃣  Re-run:                            make ci   (or make check-generated)"; \
	    echo ""; \
	    echo "   ↩️  If the changes look WRONG, undo them (git checkout -- .) and fix the"; \
	    echo "      generator or its input instead — never hand-edit a generated file."; \
	    echo ""; \
	    exit 1; \
	  fi

########################################################################################################
# Vendored sources
########################################################################################################

.PHONY: sync-common-vendored check-vendored-origin check-vendored check-cleartext-config

sync-common-vendored: ## Write every vendored destination from its source of truth
	$(call run-fhevm-npm,sync-vendored)

# Read-only: re-renders every face of sdk/cleartext-config.json in memory and fails if a committed one
# differs. The direct guard for the faces nothing else reads back — scripts/cleartext-config.sh above all.
check-cleartext-config: ## Does every generated cleartext-config face match sdk/cleartext-config.json
	$(call run-fhevm-npm,check-cleartext-config)

check-vendored-origin: ## Does each local vendored folder match its declared origin (git commit)
	$(call run-fhevm-npm,check-vendored-origin)

# The full vendored audit, read-only: copies current (would the copier write anything?) AND
# legitimate (does each vendored folder match its declared origin?). Green on both is the only state
# meaning every copy is current and accounted for. The cheaper compare runs first, to fail fast.
check-vendored: ## Audit vendored content: copies current AND matching their declared origins
	$(call run-fhevm-npm,sync-vendored --check)
	$(call run-fhevm-npm,check-vendored-origin)
