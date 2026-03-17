#!/usr/bin/env bash
set -euo pipefail

defaults=$(./fhevm-cli compat-defaults)
modern_relayer_version=$(echo "$defaults" | jq -r .externalDefaults.RELAYER_VERSION)
modern_relayer_migrate_version=$(echo "$defaults" | jq -r .externalDefaults.RELAYER_MIGRATE_VERSION)
simple_acl_cutover=$(echo "$defaults" | jq -r .anchors.SIMPLE_ACL_MIN_SHA)

stack_era="${STACK_ERA:-}"
if [ "${FORCE_MODERN_RELAYER:-false}" = "true" ]; then
  stack_era="modern"
fi

if [ -z "$stack_era" ]; then
  for version in "$@"; do
    [ -n "$version" ] || continue
    if git rev-parse -q --verify "${version}^{commit}" >/dev/null 2>&1 \
      && git merge-base --is-ancestor "$simple_acl_cutover" "$version" >/dev/null 2>&1; then
      stack_era="modern"
      break
    fi
  done
fi

write_env() {
  if [ -n "${GITHUB_ENV:-}" ]; then
    echo "$1=$2" >> "$GITHUB_ENV"
  else
    printf '%s=%s\n' "$1" "$2"
  fi
}

if [ -n "$stack_era" ]; then
  write_env STACK_ERA "$stack_era"
fi
if [ "${stack_era:-}" = "modern" ] && [ -z "${RELAYER_VERSION:-}" ]; then
  write_env RELAYER_VERSION "$modern_relayer_version"
fi
if [ "${stack_era:-}" = "modern" ] && [ -z "${RELAYER_MIGRATE_VERSION:-}" ]; then
  write_env RELAYER_MIGRATE_VERSION "$modern_relayer_migrate_version"
fi
if [ "${RELAYER_VERSION:-}" = "$modern_relayer_version" ] && [ -z "${RELAYER_MIGRATE_VERSION:-}" ]; then
  write_env RELAYER_MIGRATE_VERSION "$modern_relayer_migrate_version"
fi
