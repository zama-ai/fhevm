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
    # The specimens carry their own fixed test ids; they compile the same under every environment.
    zama_host|confidential_token|demo_vault|confidential_batcher|encrypted_counter|dep_chain) ;;
    *) echo "unknown program: $program" >&2; exit 1;;
  esac
done
bash scripts/install-sbf-tools.sh
# build.rs reads the program ids from the environment file; each program's cargo features come from
# the same file (`features.<program>`). The environment is passed as cargo config, not a shell
# variable: .cargo/config.toml pins PROGRAM_ENVIRONMENT to preview-env so a stray export cannot leak
# into other builds, and this command-line value overrides that pin for this build only.
# `anchor build -- <cargo-build-sbf args> -- <cargo args>`.
cargo_config=(--config "env.PROGRAM_ENVIRONMENT.value=\"$environment\"" --config 'env.PROGRAM_ENVIRONMENT.force=true')
for program in "$@"; do
  features=$(python3 -c 'import json, sys; print(",".join(json.load(open(sys.argv[1])).get("features", {}).get(sys.argv[2], [])))' "$environment_file" "$program")
  args=(build --ignore-keys --no-idl -p "$program" --)
  if [[ -n "$features" ]]; then args+=(--features "$features"); fi
  args+=(-- "${cargo_config[@]}")
  anchor "${args[@]}"
done
