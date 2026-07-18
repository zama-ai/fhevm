#!/usr/bin/env bash
#
# Derive private keys + addresses from a BIP-39 mnemonic using foundry's `cast`.
#
# Usage: ./derive-keys.sh [COUNT] [MNEMONIC] [PATH_PREFIX]
#   COUNT        number of indices to derive, starting at 0 (default: 10)
#   MNEMONIC     the mnemonic phrase          (default: FHEVM test mnemonic)
#   PATH_PREFIX  HD path prefix; the index is appended (default: m/44'/60'/0'/2/)
#
# Examples:
#   ./derive-keys.sh                       # first 10 keys, default mnemonic + coprocessor path
#   ./derive-keys.sh 5                     # first 5
#   ./derive-keys.sh 3 "word1 ... word12"  # custom mnemonic
#   ./derive-keys.sh 3 "word1 ... word12" "m/44'/60'/0'/0/"  # custom path
set -euo pipefail

COUNT="${1:-25}"
MNEMONIC="${2:-test test test test test test test future home engine virtual motion}"
# Default path kept as a plain assignment: the single quotes are literal inside double quotes, but
# would break bash's parser inside a `${3:-...}` default expansion.
if [ "$#" -ge 3 ]; then PATH_PREFIX="$3"; else PATH_PREFIX="m/44'/60'/0'/4/"; fi

if ! command -v cast >/dev/null 2>&1; then
  echo "error: 'cast' not found — install foundry (https://getfoundry.sh)" >&2
  exit 1
fi

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
