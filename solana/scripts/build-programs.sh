#!/usr/bin/env bash
# Compile selected programs with the same pinned toolchain in CI, Docker and local tests.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
profile="${1:?usage: build-programs.sh localnet|preview-env PROGRAM...}"
shift
[[ "$#" -gt 0 ]] || { echo 'select at least one program' >&2; exit 1; }
case "$profile" in localnet|preview-env) ;; *) echo "unknown program profile: $profile" >&2; exit 1;; esac
anchor_version=$(sed -n 's/^anchor_version = "\([^"]*\)"/\1/p' Anchor.toml)
solana_version=$(sed -n 's/^solana_version = "\([^"]*\)"/\1/p' Anchor.toml)
[[ "$(anchor --version)" == "anchor-cli $anchor_version" ]] || { echo "Anchor $anchor_version required" >&2; exit 1; }
# AVM can change the active Solana installation while resolving Anchor.
[[ "$(solana --version)" == "solana-cli $solana_version "* ]] || { echo "Solana $solana_version required" >&2; exit 1; }
for program in "$@"; do
  case "$program" in
    zama_host|confidential_token|demo_vault|confidential_batcher) ;;
    encrypted_counter|dep_chain) [[ "$profile" == localnet ]] || { echo 'specimens are local-only' >&2; exit 1; } ;;
    *) echo "unknown program: $program" >&2; exit 1;;
  esac
done
for program in "$@"; do
  args=(build --ignore-keys --no-idl -p "$program")
  if [[ "$profile" == preview-env ]]; then args+=(-- --features preview-env); fi
  anchor "${args[@]}"
done
