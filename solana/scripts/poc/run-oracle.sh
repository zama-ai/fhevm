#!/usr/bin/env bash
# L0+L1 deterministic acceptance oracle for the Solana PoC.
#
# The autonomous loop calls THIS script (not an LLM agent) to decide pass/fail,
# so the cheap, deterministic gates cost ~0 reasoning tokens. Agents are spent
# only on writing code and on adversarial verification.
#
#   L0 (cheap, always): form gate · rustfmt · clippy -D warnings · anchor build
#                       + host-listener IDL sync
#   L1 (regression):    full workspace test suite (the Mollusk floor + new tests)
#
# Run from anywhere; it resolves to solana/.  Env knobs:
#   SKIP_BUILD=1   skip the slow `anchor build`/IDL check when the change can't
#                  affect the on-chain program surface (pure off-chain Rust).
#   FORM_BASE=ref  diff base for the form gate (default origin/feature/solana).
#
# L3 (integration slice on the live docker side-stack) is a separate step — see
# HARNESS.md — because it needs the validator + fhevm-cli backend running.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/../.." # -> solana/
step() {
	echo
	echo ">>> $1"
}

step "L0 · check-form"
bash scripts/poc/check-form.sh

step "L0 · rustfmt"
cargo fmt --check

step "L0 · clippy (-D warnings)"
cargo clippy --workspace --all-targets -- -D warnings

if [[ "${SKIP_BUILD:-0}" != "1" ]]; then
	step "L0 · anchor build + host-listener IDL sync"
	bash scripts/check-zama-host-idl.sh
fi

step "L1 · workspace tests"
cargo test --workspace

echo
echo "run-oracle: ALL GREEN"
