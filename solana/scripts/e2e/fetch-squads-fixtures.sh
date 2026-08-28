#!/usr/bin/env bash
# fetch-squads-fixtures.sh — fetch the real Squads v4 program for the delegated-decrypt e2e.
#
# Writes into solana/target/squads/ (untracked):
#   squads_multisig_program.so  — dumped from mainnet, sha256-pinned
#   program_config.json         — the ProgramConfig PDA, its *data* sha256-pinned
#   treasury.json               — synthesized plain system account (the config names it; the
#                                 mainnet multisig creation fee is 0, so it only has to exist)
#
# Nothing is committed: the repo deliberately tracks no executable binaries (the only vendored
# binaries are the SDK's WASM blobs), so the artifact is fetched on demand and verified against
# the pins below. A pin mismatch means Squads upgraded the program (or reconfigured) on mainnet —
# review the upstream change (github.com/Squads-Protocol/v4, AGPL-3.0), then update the pins
# deliberately. Idempotent: valid cached fixtures are kept, nothing is re-fetched.
#
# The output directory never holds an unverified file: fetches land on a temporary path and are
# renamed in only after their hash matches, so a mismatch leaves nothing behind for a later boot
# to load unchecked. Exit codes tell the caller which failure it is: 2 is a PIN MISMATCH (loud,
# review upstream); any other nonzero exit is transport or tooling (offline is fine to skip on).
set -euo pipefail

SQUADS_PROGRAM_ID="SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf"
PROGRAM_CONFIG_ADDRESS="BSTq9w3kZwNwpBXJEvTZz2G9ZTNyKBvoSeXMvwb4cNZr"
TREASURY_ADDRESS="5DH2e3cJmFpyi6mk65EGFediunm4ui6BiKNUNrhWtD1b"

# Pinned 2026-08-27 from mainnet-beta.
SO_SHA256="dec8d3e0fae58c7c8f2416e5f67c25e673f047afd6dd2bba4a47e0b29a01d34c"
CONFIG_DATA_SHA256="ba8e7712f069f2e68e8583b105ac2eb22d791265440da56df32f9f3f734af3e0"

RPC_URL="${SQUADS_FIXTURES_RPC_URL:-https://api.mainnet-beta.solana.com}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${ROOT}/target/squads"
SO_PATH="${OUT}/squads_multisig_program.so"
CONFIG_PATH="${OUT}/program_config.json"
TREASURY_PATH="${OUT}/treasury.json"

mkdir -p "${OUT}"
# Leftovers of an interrupted fetch; never loaded (genesis names exact paths), only clutter.
rm -f "${OUT}"/*.fetching

# coreutils on Linux images, perl shasum on stock macOS — whichever this machine has.
sha256_of_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

# The pin covers the account *data* only: lamports drift with rent economics, data is the config.
config_data_sha256() {
  python3 - "$1" <<'EOF'
import base64, hashlib, json, sys
with open(sys.argv[1]) as handle:
    account = json.load(handle)
print(hashlib.sha256(base64.b64decode(account["account"]["data"][0])).hexdigest())
EOF
}

so_valid() { [[ -f "${SO_PATH}" && "$(sha256_of_file "${SO_PATH}")" == "${SO_SHA256}" ]]; }
config_valid() { [[ -f "${CONFIG_PATH}" && "$(config_data_sha256 "${CONFIG_PATH}")" == "${CONFIG_DATA_SHA256}" ]]; }

if ! so_valid; then
  echo "[squads-fixtures] dumping ${SQUADS_PROGRAM_ID} from ${RPC_URL}"
  fetched_so="${SO_PATH}.fetching"
  solana program dump -u "${RPC_URL}" "${SQUADS_PROGRAM_ID}" "${fetched_so}"
  fetched_so_sha256="$(sha256_of_file "${fetched_so}")"
  if [[ "${fetched_so_sha256}" != "${SO_SHA256}" ]]; then
    rm -f "${fetched_so}"
    echo "[squads-fixtures] PIN MISMATCH: squads_multisig_program.so hashes to ${fetched_so_sha256}," >&2
    echo "[squads-fixtures] expected ${SO_SHA256}. Squads likely upgraded on mainnet — review" >&2
    echo "[squads-fixtures] github.com/Squads-Protocol/v4 and update the pins in this script." >&2
    exit 2
  fi
  mv "${fetched_so}" "${SO_PATH}"
fi

if ! config_valid; then
  echo "[squads-fixtures] fetching ProgramConfig ${PROGRAM_CONFIG_ADDRESS}"
  fetched_config="${CONFIG_PATH}.fetching"
  solana account -u "${RPC_URL}" "${PROGRAM_CONFIG_ADDRESS}" --output json --output-file "${fetched_config}" >/dev/null
  fetched_config_sha256="$(config_data_sha256 "${fetched_config}")"
  if [[ "${fetched_config_sha256}" != "${CONFIG_DATA_SHA256}" ]]; then
    rm -f "${fetched_config}"
    echo "[squads-fixtures] PIN MISMATCH: ProgramConfig data hashes to ${fetched_config_sha256}," >&2
    echo "[squads-fixtures] expected ${CONFIG_DATA_SHA256}. The Squads program config changed on" >&2
    echo "[squads-fixtures] mainnet (authority/fee/treasury) — review and update the pins." >&2
    exit 2
  fi
  mv "${fetched_config}" "${CONFIG_PATH}"
fi

# Synthesized, not dumped: a wallet's lamports change constantly, so a dump could never be
# pinned. Only existence matters — the creation fee is 0 (asserted by the config pin above).
if [[ ! -f "${TREASURY_PATH}" ]]; then
  python3 - "${TREASURY_PATH}" "${TREASURY_ADDRESS}" <<'EOF'
import json, sys
account = {
    "pubkey": sys.argv[2],
    "account": {
        "lamports": 1_000_000_000,
        "data": ["", "base64"],
        "owner": "11111111111111111111111111111111",
        "executable": False,
        "rentEpoch": 0,
        "space": 0,
    },
}
with open(sys.argv[1], "w") as handle:
    json.dump(account, handle, indent=2)
    handle.write("\n")
EOF
fi

echo "[squads-fixtures] ready: ${OUT}"
