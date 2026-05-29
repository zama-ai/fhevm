#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IDL_DEST="$ROOT/../coprocessor/fhevm-engine/host-listener/idl/zama_host.json"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys
cp target/idl/zama_host.json "$IDL_DEST"
echo "Synced zama_host.json -> ${IDL_DEST}"
