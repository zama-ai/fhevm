#!/usr/bin/env bash
# Local macOS (Apple Silicon) smoke test for the Yellowstone gRPC geyser plugin.
#
# Why this exists:
#   The CI/production artifact bakes the plugin into an amd64 solana-test-validator
#   Docker image (Agave 2.1.21) — see ./Dockerfile + ../scripts/poc/docker-compose.solana.yml.
#   That image CANNOT run on Apple Silicon: Solana validators require x86 AVX, which
#   Rosetta (used by Docker/OrbStack to run amd64 here) does not provide, so the
#   validator aborts on startup ("Incompatible CPU detected: missing AVX support").
#
#   Locally we therefore go fully native arm64: build the Yellowstone plugin as a
#   macOS .dylib pinned to the LOCAL validator's 3.1.x line, and load it into the
#   native solana-test-validator already installed on this machine. This validates
#   the gRPC mechanism locally; the exact 2.1.21 amd64 artifact is validated on
#   x86_64 hardware via the Docker path.
set -euo pipefail

# Yellowstone tag — must match the local validator's Agave line (ABI-coupled).
# Local validator is 3.1.x; the geyser ABI is stable across 3.1 patch releases.
REF="${YELLOWSTONE_REF:-v12.1.0+solana.3.1.10}"
WORK="${WORK:-$HOME/.cache/yellowstone-macos}"
GRPC_ADDR="${GRPC_ADDR:-127.0.0.1:10000}"
GRPC_PORT="${GRPC_ADDR##*:}"
RPC_PORT="${RPC_PORT:-8899}"
LEDGER="${LEDGER:-/tmp/yellowstone-smoke-ledger}"
KEEP="${KEEP:-0}"   # KEEP=1 leaves the validator running after the checks

echo "==> local validator: $(solana-test-validator --version)"

# 1. Build the plugin as a native macOS .dylib.
mkdir -p "$WORK"
if [ ! -e "$WORK/src/Cargo.toml" ]; then
  echo "==> cloning yellowstone-grpc $REF"
  rm -rf "$WORK/src"
  git clone --depth 1 --branch "$REF" https://github.com/rpcpool/yellowstone-grpc "$WORK/src"
fi
echo "==> building yellowstone-grpc-geyser (.dylib, native arm64)…"
( cd "$WORK/src" && cargo build --release -p yellowstone-grpc-geyser )
DYLIB="$WORK/src/target/release/libyellowstone_grpc_geyser.dylib"
test -f "$DYLIB" || { echo "ERROR: .dylib not produced at $DYLIB" >&2; exit 1; }
echo "==> plugin: $DYLIB"

# 2. Generate the geyser config (absolute libpath; machine-specific, not committed).
CFG="$WORK/yellowstone-config.json"
cat > "$CFG" <<JSON
{
  "libpath": "$DYLIB",
  "log": { "level": "info" },
  "grpc": { "address": "$GRPC_ADDR" }
}
JSON
echo "==> config: $CFG"

# 3. Launch the native validator with the plugin (background).
rm -rf "$LEDGER"
echo "==> starting native solana-test-validator + plugin…"
solana-test-validator --reset --rpc-port "$RPC_PORT" --ledger "$LEDGER" \
  --geyser-plugin-config "$CFG" > "$WORK/run.log" 2>&1 &
VPID=$!
trap '[ "$KEEP" = "1" ] || kill "$VPID" 2>/dev/null || true' EXIT

# 4. Wait for RPC health.
echo "==> waiting for RPC health…"
healthy=no
for _ in $(seq 1 60); do
  if solana cluster-version -u "http://127.0.0.1:$RPC_PORT" >/dev/null 2>&1; then healthy=yes; break; fi
  if ! kill -0 "$VPID" 2>/dev/null; then
    echo "ERROR: validator exited early; last log lines:" >&2
    tail -25 "$LEDGER/validator.log" 2>/dev/null || tail -25 "$WORK/run.log" >&2
    exit 1
  fi
  sleep 2
done
[ "$healthy" = yes ] || { echo "ERROR: RPC not healthy in time" >&2; tail -25 "$LEDGER/validator.log" 2>/dev/null; exit 1; }
echo "    RPC healthy."

# 5. Plugin-load + gRPC checks.
echo "==> plugin-load / gRPC lines in validator log:"
grep -iE "geyser|plugin|grpc|started server" "$LEDGER/validator.log" 2>/dev/null | head -10 || true

echo "==> gRPC endpoint check ($GRPC_ADDR):"
if command -v grpcurl >/dev/null 2>&1; then
  grpcurl -plaintext "127.0.0.1:$GRPC_PORT" geyser.Geyser/GetVersion 2>&1 | head -5 || \
    echo "    (GetVersion via reflection failed — yellowstone may not expose reflection; relying on port check)"
fi
if nc -z 127.0.0.1 "$GRPC_PORT" 2>/dev/null; then
  echo "    port $GRPC_PORT is listening ✓"
else
  echo "ERROR: gRPC port $GRPC_PORT not listening" >&2
  exit 1
fi

echo "==> SMOKE PASS: validator up, plugin loaded, gRPC port $GRPC_PORT listening."
if [ "$KEEP" = "1" ]; then
  echo "==> KEEP=1: validator left running (pid $VPID), gRPC at $GRPC_ADDR. Stop with: kill $VPID"
  wait "$VPID"
fi
