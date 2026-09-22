# No selector guard here, on purpose. protocol-apps' shared contracts/Makefile
# excluded this package from its selector list -- "solanaOFT is excluded because
# it duplicates contracts already emitted by the token package" -- and that still
# holds: ZamaERC20, ZamaOFT and ZamaOFTAdapter are the token package's contracts,
# guarded by ../token/selectors.txt. A second copy here would drift or duplicate.

# ── CI entry points ─────────────────────────────────────────────────────────
# Every step of this package's workflow is `make <target>`, so the same command
# runs locally and in CI. These delegate to package.json rather than restating
# the command, so there is one source of truth for what runs and one entry point
# for how to run it.
#
# NOTE `lint-check` not `lint`: the convention across protocol-contracts, because
# in some packages a `lint` target runs the --fix variants and REWRITES files,
# which is the wrong thing for CI.
#
# `compile` and `test` are the forge halves only -- exactly what protocol-apps'
# CI ran (`pnpm run lint`, `pnpm run test:forge`). Two halves do not run in CI:
#
#   - compile:anchor / test:anchor need `anchor` 0.31.1 and a Solana BPF
#     toolchain. No runner in this repository installs either, and neither did
#     protocol-apps'. The scripts stay in package.json for a machine that has
#     them.
#   - compile:hardhat / test:hardhat fail on a fresh checkout: tasks/evm/sendEvm.ts
#     imports ../../layerzero.config, a per-operator file that .gitignore
#     excludes (only the .mainnet and .testnet variants are tracked).
install:
	pnpm install --frozen-lockfile

lint-check:
	pnpm run lint

compile:
	pnpm run compile:forge

test:
	pnpm run test:forge

# `test` is phony so the test/ directory cannot satisfy it.
.PHONY: install lint-check compile test
