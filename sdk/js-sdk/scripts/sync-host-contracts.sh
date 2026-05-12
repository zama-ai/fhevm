#!/usr/bin/env bash
#
# Syncs <repo-root>/host-contracts/contracts/
#   into <repo-root>/sdk/js-sdk/contracts/src/host-contracts/
#
# Also ensures a dummy FHEVMHostAddresses.sol exists at
# <repo-root>/sdk/js-sdk/contracts/src/addresses/FHEVMHostAddresses.sol
# so the synced tree can be compiled. The dummy is created only if the
# file is absent — it will not overwrite a previously generated one.
#
# Behaviour:
#   - Destination missing              : copy recursively.
#   - Destination exists, identical    : no-op for the copy step.
#   - Destination exists, differs      : print drift, exit 1.
#   - Dummy addresses file missing     : create with address(0) placeholders.
#   - Dummy addresses file present     : leave untouched.
#
# Works regardless of the cwd from which the script is invoked.

set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
SRC="$ROOT/host-contracts/contracts"
DST="$ROOT/sdk/js-sdk/contracts/src/host-contracts/contracts"
ADDRESSES_FILE="$ROOT/sdk/js-sdk/contracts/src/host-contracts/addresses/FHEVMHostAddresses.sol"

write_dummy_addresses_file() {
  if [ -f "$ADDRESSES_FILE" ]; then
    return 0
  fi
  mkdir -p "$(dirname "$ADDRESSES_FILE")"
  cat > "$ADDRESSES_FILE" <<'EOF'
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DUMMY placeholder addresses. Replace by running:
//   forge script script/ComputeAddresses.s.sol

address constant aclAdd = address(0);
address constant fhevmExecutorAdd = address(0);
address constant kmsVerifierAdd = address(0);
address constant inputVerifierAdd = address(0);
address constant hcuLimitAdd = address(0);
address constant pauserSetAdd = address(0);
EOF
  echo "Wrote dummy addresses file: $ADDRESSES_FILE"
}

if [ ! -d "$SRC" ]; then
  echo "ERROR: source directory not found: $SRC" >&2
  exit 1
fi

if [ ! -d "$DST" ]; then
  echo "Destination does not exist, copying tree..."
  mkdir -p "$(dirname "$DST")"
  cp -R "$SRC" "$DST"
  echo "Copy complete: $DST"
  write_dummy_addresses_file
  exit 0
fi

# Destination exists — compare trees byte-for-byte.
# diff -rq: recursive, brief (reports only which files differ).
# Exit status 0 = identical, 1 = differences, 2 = error.
if diff -rq "$SRC" "$DST" > /dev/null 2>&1; then
  echo "Destination is identical to source."
  write_dummy_addresses_file
  exit 0
fi

{
  echo "ERROR: sync drift between:"
  echo "  source: $SRC"
  echo "  dest:   $DST"
  echo
  echo "Differing or one-sided files:"
  diff -rq "$SRC" "$DST" || true
  echo
  echo "Delete '$DST' and re-run this script, or reconcile manually."
} >&2
exit 1
