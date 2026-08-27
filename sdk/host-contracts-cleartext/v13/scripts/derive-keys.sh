#!/usr/bin/env bash
#
# Derive private keys + addresses from a BIP-39 mnemonic using foundry's `cast`.
#
# Usage: ./derive-keys.sh [COUNT] [MNEMONIC] [PATH_PREFIX]
#   COUNT        number of indices to derive, starting at 0 (default: 10)
#   MNEMONIC     the mnemonic phrase          (default: FHEVM_MNEMONIC)
#   PATH_PREFIX  HD path prefix; the index is appended
#                (default: CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_PATH)
#
# Both defaults are read from sdk/cleartext-config.json rather than written here, so
# they cannot drift from the pools the stack actually registers. Named rather than quoted in this help
# text for the same reason: a literal here would be a fourth copy.
#
# Examples:
#   ./derive-keys.sh                       # first 10 keys, default mnemonic + coprocessor path
#   ./derive-keys.sh 5                     # first 5
#   ./derive-keys.sh 3 "word1 ... word12"  # custom mnemonic
#   ./derive-keys.sh 3 "word1 ... word12" "m/44'/60'/0'/0/"  # custom path
set -euo pipefail

# shellcheck source=scripts/cleartext-config-lib.sh
source "${BASH_SOURCE[0]%/*}/cleartext-config-lib.sh"

if ! command -v cast >/dev/null 2>&1; then
  echo "error: 'cast' not found — install foundry (https://getfoundry.sh)" >&2
  exit 1
fi

COUNT="${1:-25}"
MNEMONIC="${2:-$(cfg_constant FHEVM_MNEMONIC)}"
PATH_PREFIX="${3:-$(cfg_constant CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_PATH)}"

addrs=()
pks=()
for ((i = 0; i < COUNT; i++)); do
  path="${PATH_PREFIX}${i}"
  addrs+=("$(cast wallet address --mnemonic "$MNEMONIC" --mnemonic-derivation-path "$path")")
  pks+=("$(cast wallet private-key --mnemonic "$MNEMONIC" --mnemonic-derivation-path "$path")")
done

echo "export const DEFAULT_COPROCESSOR_PK = ["
for pk in "${pks[@]}"; do printf "  '%s',\n" "$pk"; done
echo "];"
echo
echo "export const DEFAULT_COPROCESSOR_ADDRESSES = ["
for addr in "${addrs[@]}"; do printf "  '%s',\n" "$addr"; done
echo "];"
