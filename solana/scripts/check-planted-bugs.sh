#!/usr/bin/env bash
# check-planted-bugs.sh — prove the capability property test catches the bugs it exists for.
#
# Usage (from solana/, after check-zama-host-idl.sh built target/deploy):
#   bash scripts/check-planted-bugs.sh
#
# Each patch in runtime-tests/planted-bugs/ removes one authorization check from zama-host. For
# each, this applies the patch, rebuilds the default artifact and requires `capability_invariants`
# to fail on the invariant the file name starts with (h1- or h2-). On exit it reverts the patch
# and rebuilds the unpatched artifact.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

build_host() {
  NO_DNA=1 anchor build --ignore-keys --no-idl -p zama_host
}

applied=""
restore() {
  if [[ -n "$applied" ]]; then
    git apply -R "$applied"
  fi
  build_host
}
trap restore EXIT

log="$(mktemp)"
for patch in runtime-tests/planted-bugs/*.patch; do
  invariant="$(basename "$patch" | cut -d- -f1 | tr '[:lower:]' '[:upper:]')"
  git apply "$patch"
  applied="$patch"
  build_host
  if cargo test -p zama-solana-runtime-tests --test capability_invariants \
    only_the_admin_changes_trust_roots_and_only_an_authority_changes_its_store >"$log" 2>&1; then
    echo "$patch: the property test passed with the bug planted" >&2
    exit 1
  fi
  if ! grep -q "$invariant: " "$log"; then
    tail -n 40 "$log" >&2
    echo "$patch: the property test failed, but not on $invariant" >&2
    exit 1
  fi
  echo "$patch: caught by $invariant"
  git apply -R "$patch"
  applied=""
done
