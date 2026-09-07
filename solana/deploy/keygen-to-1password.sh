#!/usr/bin/env bash
# Generate (or reuse) the three preview-env Solana keypairs, keep them gitignored
# under solana/deploy/profiles/preview-env/, and attach them to a 1Password item.
#
# Private JSON is never printed. `--silent` suppresses the BIP39 seed phrase.
# Do NOT run under `bash -x`. Do NOT pipe solana-keygen stdout into `op`.
#
# Usage (from a terminal that can approve the 1Password desktop prompt):
#   VAULT="Preview Env" ./solana/deploy/keygen-to-1password.sh
# Laptop-only. CI does not use 1Password.
set -euo pipefail
if [[ $- == *x* ]]; then
  echo "refusing to run with xtrace enabled" >&2
  exit 1
fi
umask 077

VAULT="${VAULT:-Preview Env}"
TITLE="${TITLE:-preview-env - Solana host keypairs (public Solana cluster)}"
TAGS="${TAGS:-preview-env,solana,fhevm}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROFILE_DIR="${SCRIPT_DIR}/profiles/preview-env"
mkdir -p "$PROFILE_DIR"

need() { command -v "$1" >/dev/null || { echo "missing $1" >&2; exit 1; }; }
need solana-keygen
need op

pubkey_of() { solana-keygen pubkey "$1"; }

ensure_keypair() {
  local name="$1"
  local out="${PROFILE_DIR}/${name}-keypair.json"
  if [[ -f "$out" ]]; then
    pubkey_of "$out"
    return
  fi
  solana-keygen new --no-bip39-passphrase --silent --outfile "$out" >/dev/null
  chmod 600 "$out"
  pubkey_of "$out"
}

zama_host_pk="$(ensure_keypair zama_host)"
confidential_token_pk="$(ensure_keypair confidential_token)"
deployer_pk="$(ensure_keypair deployer)"

echo "public keys (safe to commit; private JSON stays in ${PROFILE_DIR} and 1Password)"
echo "  zama_host:           ${zama_host_pk}"
echo "  confidential_token:  ${confidential_token_pk}"
echo "  deployer:            ${deployer_pk}"

if ! item_id="$(
  op item create --category=secureNote \
    --title="$TITLE" \
    --vault="$VAULT" \
    --tags="$TAGS" \
    "zama_host.json[file]=${PROFILE_DIR}/zama_host-keypair.json" \
    "confidential_token.json[file]=${PROFILE_DIR}/confidential_token-keypair.json" \
    "deployer.json[file]=${PROFILE_DIR}/deployer-keypair.json" \
    "zama_host_pubkey[text]=${zama_host_pk}" \
    "confidential_token_pubkey[text]=${confidential_token_pk}" \
    "deployer_pubkey[text]=${deployer_pk}" \
    "purpose[text]=Shared fhevm preview-env Solana host on public Solana devnet. Deployer is upgrade authority and HostConfig.admin. Program JSON is first-deploy only. NOT official Zama protocol devnet." \
    --format=json \
    | python3 -c 'import json,sys; print(json.load(sys.stdin)["id"])'
)"; then
  echo "1Password item create failed — approve the desktop app prompt and re-run (existing keypairs will be reused)." >&2
  exit 1
fi

cat <<EOF
created 1Password item
  vault:  ${VAULT}
  title:  ${TITLE}
  id:     ${item_id}

extract later with:
  ./solana/deploy/fetch-preview-env-keypairs.sh
EOF
