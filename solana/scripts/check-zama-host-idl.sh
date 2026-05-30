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

# The IDL diff above only covers structure; EVENT_VERSION is a runtime u8 stamped
# on every event and independently re-declared by the host-listener decoder. If the
# two drift, the listener silently drops every event (fail-closed but a silent
# ingestion outage), so assert they match here.
HOST_STATE="$ROOT/programs/zama-host/src/state.rs"
LISTENER_BUILD="$ROOT/../coprocessor/fhevm-engine/host-listener/build.rs"
HOST_EVENT_VERSION="$(sed -n 's/.*pub const EVENT_VERSION: u8 = \([0-9]*\);.*/\1/p' "$HOST_STATE" | head -1)"
LISTENER_EVENT_VERSION="$(sed -n 's/.*pub const EVENT_VERSION: u8 = \([0-9]*\);.*/\1/p' "$LISTENER_BUILD" | head -1)"
if [ -z "$HOST_EVENT_VERSION" ] || [ -z "$LISTENER_EVENT_VERSION" ]; then
  echo "error: could not read EVENT_VERSION from host state.rs and/or host-listener build.rs" >&2
  exit 1
fi
if [ "$HOST_EVENT_VERSION" != "$LISTENER_EVENT_VERSION" ]; then
  echo "error: EVENT_VERSION mismatch: zama-host state.rs=$HOST_EVENT_VERSION vs host-listener build.rs=$LISTENER_EVENT_VERSION" >&2
  echo "       bump both together; the listener silently drops version-mismatched events." >&2
  exit 1
fi
echo "EVENT_VERSION host=$HOST_EVENT_VERSION listener=$LISTENER_EVENT_VERSION in sync"
