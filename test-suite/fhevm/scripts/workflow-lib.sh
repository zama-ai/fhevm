#!/usr/bin/env bash

normalize_csv() {
  printf '%s\n' "$@" |
    tr ',' '\n' |
    sed 's/^[[:space:]]*//; s/[[:space:]]*$//' |
    sed '/^$/d' |
    awk '!seen[$0]++' |
    paste -sd, -
}

resolve_compat_test() {
  local value="$1"
  local workspace_root="${GITHUB_WORKSPACE:-$(git rev-parse --show-toplevel)}"
  local workspace_value="${workspace_root}/${value}"
  local compat_dir="${workspace_root}/test-suite/fhevm/compat-tests"
  if [ -f "$value" ]; then
    realpath "$value"
    return
  fi
  if [ -f "$workspace_value" ]; then
    realpath "$workspace_value"
    return
  fi
  if [ -f "${compat_dir}/${value}.json" ]; then
    realpath "${compat_dir}/${value}.json"
    return
  fi
  if [ -f "${compat_dir}/${value}" ]; then
    realpath "${compat_dir}/${value}"
    return
  fi
  echo "Could not resolve compat-test: $value" >&2
  return 1
}

append_override_args() {
  local normalized
  normalized="$(normalize_csv "$@")"
  if [ -z "$normalized" ]; then
    return
  fi
  local override_groups
  IFS=',' read -r -a override_groups <<< "$normalized"
  for group in "${override_groups[@]}"; do
    args+=(--override "$group")
  done
}
