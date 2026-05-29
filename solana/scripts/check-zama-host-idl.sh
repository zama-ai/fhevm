#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IDL_DEST="$ROOT/../coprocessor/fhevm-engine/host-listener/idl/zama_host.json"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys

if ! diff -u "$IDL_DEST" target/idl/zama_host.json; then
  echo "error: host-listener IDL is out of sync; run solana/scripts/sync-zama-host-idl.sh" >&2
  exit 1
fi

echo "zama_host.json is in sync"
