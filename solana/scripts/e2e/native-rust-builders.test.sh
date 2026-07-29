#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091 # The script path is resolved above.
SOLANA_E2E_LIBRARY_ONLY=1 source "$SCRIPT_DIR/clean-e2e.sh" >/dev/null

CALL_LOG="$(mktemp)"
trap 'rm -f "$CALL_LOG"' EXIT
FAIL_REMOVE=""
FAIL_TAG_TARGET=""
docker() {
  printf '%s\n' "$*" >> "$CALL_LOG"
  if [ "$1 $2" = "image inspect" ]; then
    echo "sha256:native"
    return 0
  fi
  if [ "$1 $2" = "image tag" ] && [ "${4:-}" = "$FAIL_TAG_TARGET" ]; then
    return 1
  fi
  if [ "$1 $2" = "image rm" ] && [ "${3:-}" = "$FAIL_REMOVE" ]; then
    return 1
  fi
}

assert_call() {
  local expected="$1"
  grep -Fqx "$expected" "$CALL_LOG" || {
    echo "missing Docker call: $expected" >&2
    return 1
  }
}

# A cleanup failure for one alias must not leave later aliases behind.
# shellcheck disable=SC2034 # These arrays are consumed by the sourced cleanup function.
NATIVE_RUST_BUILDER_ALIASES=("alias-one" "alias-two")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_BACKUPS=("" "")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_IMAGE_IDS=("sha256:native" "sha256:native")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_PREEXISTED=("no" "no")
FAIL_REMOVE="alias-one"
set +e
cleanup_native_rust_builder_aliases
cleanup_status=$?
set -e
[ "$cleanup_status" -eq 1 ]
assert_call "image rm alias-one"
assert_call "image rm alias-two"

# An identical tag that existed before the run must be preserved.
: > "$CALL_LOG"
FAIL_REMOVE=""
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_ALIASES=("existing-alias")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_BACKUPS=("")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_IMAGE_IDS=("sha256:native")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_PREEXISTED=("yes")
cleanup_native_rust_builder_aliases
if grep -Fqx "image rm existing-alias" "$CALL_LOG"; then
  echo "pre-existing alias was removed" >&2
  exit 1
fi

# A different prior image must be restored and its backup tag removed.
: > "$CALL_LOG"
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_ALIASES=("official-alias")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_BACKUPS=("backup-alias")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_IMAGE_IDS=("sha256:native")
# shellcheck disable=SC2034
NATIVE_RUST_BUILDER_PREEXISTED=("yes")
cleanup_native_rust_builder_aliases
assert_call "image tag backup-alias official-alias"
assert_call "image rm backup-alias"

# A backup created before alias installation must not leak if the install fails.
: > "$CALL_LOG"
FAIL_TAG_TARGET="official-alias"
set +e
install_native_rust_builder_alias \
  "official-alias" \
  "native-cache" \
  "backup-alias" \
  "sha256:native" \
  "yes"
install_status=$?
set -e
[ "$install_status" -eq 1 ]
assert_call "image tag native-cache official-alias"
assert_call "image rm backup-alias"
[ "${#NATIVE_RUST_BUILDER_ALIASES[@]}" -eq 0 ]
