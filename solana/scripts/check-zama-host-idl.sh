#!/usr/bin/env bash
# check-zama-host-idl.sh — rebuild SBF artifacts and verify host IDL/ABI goldens.
#
# Usage (from solana/):
#   bash scripts/check-zama-host-idl.sh
#
# When: before Mollusk runtime tests; what CI runs for IDL/ABI sync checks.
# Writes: target/deploy only (does not update goldens; see sync-zama-host-idl.sh).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"
# cargo-build-sbf can exit successfully after reporting a stack-limit error.
# Such artifacts may execute with corrupted CPI arguments; never run tests on them.
build_log="$(mktemp)"
trap 'rm -f "$build_log"' EXIT
bash "$ROOT/scripts/install-sbf-tools.sh"
# `close_owned_accounts` exists only behind `admin-sweep` (enabled by `preview-env.json`).
# Mollusk covers it from this artifact alone; every other suite runs the default build below.
NO_DNA=1 anchor build --ignore-keys --no-idl -p zama_host -- --features admin-sweep 2>&1 | tee "$build_log"
mv target/deploy/zama_host.so target/deploy/zama_host_admin_sweep.so
NO_DNA=1 anchor build --ignore-keys 2>&1 | tee -a "$build_log"
if rg -n 'Error:.*[Ss]tack offset' "$build_log"; then
  echo "SBF stack limit exceeded" >&2
  exit 1
fi

python3 scripts/check_solana_abi.py --root "$ROOT"

# Runtime Mollusk tests load ignored SBF artifacts from target/deploy, and the build above already
# produced them on the default feature set, so Mollusk runs against the same artifact that ships.
# The one exception is `zama_host_admin_sweep.so` above, which only the sweep tests load.

# Event-version constants are runtime u8s stamped on protocol events. The ABI
# golden manifest (check_solana_abi.py above) pins both programs' versions from
# their constants.rs; the host-listener's decoded op records use
# zama_host::EVENT_VERSION directly, so no separate listener constant can drift.
