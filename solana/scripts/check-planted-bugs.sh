#!/usr/bin/env bash
# check-planted-bugs.sh — prove the capability property test catches the bugs it exists for.
#
# Usage (from solana/, after check-zama-host-idl.sh built target/deploy):
#   bash scripts/check-planted-bugs.sh
#
# Each patch in runtime-tests/planted-bugs/ removes one authorization check from zama-host and
# starts with an `Expect:` line naming the invariant and action it must fail on, such as
# `Expect: H1: Unpause`. For each, this applies the patch, rebuilds the default artifact and
# requires `capability_invariants` to fail with that message. On exit it reverts the patch and
# rebuilds the unpatched artifact.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

build_host() {
  NO_DNA=1 anchor build --ignore-keys --no-idl -p zama_host
}

log="$(mktemp)"
applied=""
restore() {
  rm -f "$log"
  if [[ -n "$applied" ]]; then
    git apply -R "$applied"
  fi
  build_host
}
trap restore EXIT

for patch in runtime-tests/planted-bugs/*.patch; do
  expected="$(sed -n 's/^Expect: //p' "$patch")"
  if [[ -z "$expected" ]]; then
    echo "$patch: no Expect: line" >&2
    exit 1
  fi
  git apply "$patch"
  applied="$patch"
  build_host
  if cargo test -p zama-solana-runtime-tests --test capability_invariants \
    only_the_admin_changes_trust_roots_and_only_an_authority_changes_its_store >"$log" 2>&1; then
    echo "$patch: the property test passed with the bug planted" >&2
    exit 1
  fi
  if ! grep -qF "Test failed: $expected" "$log"; then
    tail -n 40 "$log" >&2
    echo "$patch: the property test failed, but not with \"$expected\"" >&2
    exit 1
  fi
  echo "$patch: caught ($expected)"
  git apply -R "$patch"
  applied=""
done
