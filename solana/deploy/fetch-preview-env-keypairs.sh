#!/usr/bin/env bash
# Pull preview-env keypairs from 1Password into the gitignored profile directory.
# Laptop-only (CI uses AWS Secrets Manager). Never prints JSON.
#
# Usage:
#   VAULT="Preview Env" ./solana/deploy/fetch-preview-env-keypairs.sh
set -euo pipefail
if [[ $- == *x* ]]; then
  echo "refusing to run with xtrace enabled" >&2
  exit 1
fi
umask 077

VAULT="${VAULT:-Preview Env}"
TITLE="${TITLE:-preview-env - Solana host keypairs (public Solana cluster)}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROFILE_DIR="${SCRIPT_DIR}/profiles/preview-env"
mkdir -p "$PROFILE_DIR"

need() { command -v "$1" >/dev/null || { echo "missing $1" >&2; exit 1; }; }
need op

# Titles with '(' break `op://` refs. Resolve the item id (or pass OP_ITEM).
item_id="${OP_ITEM:-}"
if [[ -z "$item_id" ]]; then
  item_id="$(op item get "$TITLE" --vault "$VAULT" --format json | python3 -c 'import json,sys; print(json.load(sys.stdin)["id"])')"
fi

# Attachments are stored as file name "json" under a section per key
# (zama_host / confidential_token / deployer).
pull() {
  local section="$1"
  local dest="$2"
  op read --out-file "$dest" "op://${VAULT}/${item_id}/${section}/json"
  chmod 600 "$dest"
}

pull zama_host "${PROFILE_DIR}/zama_host-keypair.json"
pull confidential_token "${PROFILE_DIR}/confidential_token-keypair.json"
pull deployer "${PROFILE_DIR}/deployer-keypair.json"
# Optional until the demo keys have been added to the vault item.
if [[ "${INCLUDE_DEMOS:-false}" == true ]]; then
  pull demo_vault "${PROFILE_DIR}/demo_vault-keypair.json"
  pull confidential_batcher "${PROFILE_DIR}/confidential_batcher-keypair.json"
fi

echo "wrote keypairs to ${PROFILE_DIR} (gitignored)"
if command -v solana-keygen >/dev/null; then
  echo "  zama_host:           $(solana-keygen pubkey "${PROFILE_DIR}/zama_host-keypair.json")"
  echo "  confidential_token:  $(solana-keygen pubkey "${PROFILE_DIR}/confidential_token-keypair.json")"
  echo "  deployer:            $(solana-keygen pubkey "${PROFILE_DIR}/deployer-keypair.json")"
fi
