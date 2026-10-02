#!/usr/bin/env bash
# check-zama-host-idl.sh — rebuild SBF artifacts and verify host IDL/ABI goldens and the
# authority table.
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
for program in zama_host confidential_token confidential_batcher demo_vault; do
  NO_DNA=1 anchor build --ignore-keys --no-idl -p "$program" -- --features admin-sweep 2>&1 | tee -a "$build_log"
  mv "target/deploy/$program.so" "target/deploy/${program}_admin_sweep.so"
done
# `cleartext` records plaintexts in host accounts for the local simulator (src/cleartext). It is a
# test artifact only: build-programs.sh refuses it in environment files and deploy refuses its marker.
NO_DNA=1 bash scripts/build-programs.sh preview-env zama_host_cleartext 2>&1 | tee -a "$build_log"
NO_DNA=1 anchor build --ignore-keys 2>&1 | tee -a "$build_log"
if grep -En 'Error:.*([Ss]tack offset|overflows the maximum allowed)' "$build_log"; then
  echo "SBF stack limit exceeded" >&2
  exit 1
fi

# The deployer refuses any binary carrying the cleartext build's marker, so the marker must be in
# the cleartext build and never in the production one. The deployer copies it from the host's
# `MAGIC`; the SDK reads it from `hostConstants.ts`, which a zama-host test renders.
marker="$(sed -n 's/^pub const MAGIC: \[u8; 32\] = \*b"\(.*\)";$/\1/p' programs/zama-host/src/cleartext/layout.rs)"
[[ -n "$marker" ]] || { echo 'cannot read MAGIC from src/cleartext/layout.rs' >&2; exit 1; }
grep -qF "'$marker'" deploy/src/deploy-programs.ts ||
  { echo "deploy/src/deploy-programs.ts does not carry the cleartext marker '$marker'" >&2; exit 1; }
LC_ALL=C grep -qaF "$marker" target/deploy/zama_host_cleartext.so || { echo 'cleartext build lacks its marker' >&2; exit 1; }
if LC_ALL=C grep -qaF "$marker" target/deploy/zama_host.so; then
  echo 'production zama_host.so carries the cleartext marker' >&2
  exit 1
fi

python3 scripts/check_solana_abi.py --root "$ROOT"
python3 scripts/authority_table.py --root "$ROOT"

# Runtime Mollusk tests load ignored SBF artifacts from target/deploy, and the build above already
# produced them on the default feature set, so Mollusk runs against the same artifact that ships.
# The exceptions are the `*_admin_sweep.so` artifacts above, loaded only by cleanup tests.

# Event-version constants are runtime u8s stamped on protocol events. The ABI
# golden manifest (check_solana_abi.py above) pins both programs' versions from
# their constants.rs; the host-listener's decoded op records use
# zama_host::EVENT_VERSION directly, so no separate listener constant can drift.
