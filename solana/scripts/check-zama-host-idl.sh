#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"
NO_DNA=1 anchor build --ignore-keys

python3 scripts/check_solana_abi.py --root "$ROOT"

# Runtime Mollusk tests load ignored SBF artifacts from target/deploy. Keep the
# production IDL/ABI check above on the default feature set, then rebuild the
# confidential-token artifact with its PoC-only receiver helpers enabled. The
# host intentionally has no PoC feature or alternate verification path.
NO_DNA=1 anchor build --ignore-keys --no-idl -p confidential_token -- --features poc

# The IDL diff above only covers structure; event-version constants are runtime
# u8s stamped on protocol events and independently used by the host-listener's
# generated semantic value types. Keep the source ABI and generated types aligned.
HOST_STATE="$ROOT/programs/zama-host/src/constants.rs"
TOKEN_STATE="$ROOT/programs/confidential-token/src/constants.rs"
LISTENER_BUILD="$ROOT/../coprocessor/fhevm-engine/host-listener/build.rs"
HOST_EVENT_VERSION="$(sed -n 's/.*pub const EVENT_VERSION: u8 = \([0-9]*\);.*/\1/p' "$HOST_STATE" | head -1)"
TOKEN_EVENT_VERSION="$(sed -n 's/.*pub const APP_EVENT_VERSION: u8 = \([0-9]*\);.*/\1/p' "$TOKEN_STATE" | head -1)"
LISTENER_EVENT_VERSION="$(sed -n 's/.*pub const EVENT_VERSION: u8 = \([0-9]*\);.*/\1/p' "$LISTENER_BUILD" | head -1)"
if [ -z "$HOST_EVENT_VERSION" ] || [ -z "$TOKEN_EVENT_VERSION" ] || [ -z "$LISTENER_EVENT_VERSION" ]; then
  echo "error: could not read event versions from Solana constants.rs and/or host-listener build.rs" >&2
  exit 1
fi
if [ "$HOST_EVENT_VERSION" != "$LISTENER_EVENT_VERSION" ]; then
  echo "error: EVENT_VERSION mismatch: zama-host constants.rs=$HOST_EVENT_VERSION vs host-listener build.rs=$LISTENER_EVENT_VERSION" >&2
  echo "       bump both together so generated semantic values match the host ABI." >&2
  exit 1
fi
if [ "$TOKEN_EVENT_VERSION" != "$LISTENER_EVENT_VERSION" ]; then
  echo "error: APP_EVENT_VERSION mismatch: confidential-token constants.rs=$TOKEN_EVENT_VERSION vs host-listener build.rs=$LISTENER_EVENT_VERSION" >&2
  echo "       bump both together so generated semantic values match the token ABI." >&2
  exit 1
fi
echo "EVENT_VERSION host=$HOST_EVENT_VERSION token=$TOKEN_EVENT_VERSION listener=$LISTENER_EVENT_VERSION in sync"
