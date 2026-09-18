#!/usr/bin/env bash
# Compile selected programs with the same pinned toolchain in CI, Docker and local tests.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
environment="${1:?usage: build-programs.sh ENVIRONMENT PROGRAM...}"
shift
[[ "$#" -gt 0 ]] || { echo 'select at least one program' >&2; exit 1; }
environment_file="environments/$environment.json"
[[ -f "$environment_file" ]] || { echo "unknown environment: $environment (no $environment_file)" >&2; exit 1; }
anchor_version=$(sed -n 's/^anchor_version = "\([^"]*\)"/\1/p' Anchor.toml)
solana_version=$(sed -n 's/^solana_version = "\([^"]*\)"/\1/p' Anchor.toml)
[[ "$(anchor --version)" == "anchor-cli $anchor_version" ]] || { echo "Anchor $anchor_version required" >&2; exit 1; }
# AVM can change the active Solana installation while resolving Anchor.
[[ "$(solana --version)" == "solana-cli $solana_version "* ]] || { echo "Solana $solana_version required" >&2; exit 1; }
for program in "$@"; do
  case "$program" in
    zama_host|confidential_token|demo_vault|confidential_batcher) ;;
    encrypted_counter|dep_chain) [[ "$environment" == localnet ]] || { echo 'specimens are local-only' >&2; exit 1; } ;;
    *) echo "unknown program: $program" >&2; exit 1;;
  esac
done
bash scripts/install-sbf-tools.sh
# build.rs reads the program ids from the environment file; its cargo features come from the same file.
export PROGRAM_ENVIRONMENT="$environment"
features=$(python3 -c 'import json, sys; print(",".join(json.load(open(sys.argv[1])).get("features", [])))' "$environment_file")
for program in "$@"; do
  args=(build --ignore-keys --no-idl -p "$program")
  if [[ -n "$features" ]]; then args+=(-- --features "$features"); fi
  anchor "${args[@]}"
done
