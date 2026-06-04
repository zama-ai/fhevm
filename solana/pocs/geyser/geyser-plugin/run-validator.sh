#!/usr/bin/env bash
# Build the program + the Geyser tracker plugin, then launch a local
# solana-test-validator that loads our program and the plugin.
#
# The plugin streams two kinds of events (to stderr and to geyser-events.log):
#   - CPI / direct calls to the tracked program
#   - account writes owned by the tracked program (its PDAs)
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLUGIN_DIR="$REPO_ROOT/geyser-plugin"

# The geyser program is the one we track; caller only CPIs into it.
PROGRAM_ID="H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8"
PROGRAM_SO="$REPO_ROOT/target/deploy/geyser.so"
CALLER_ID="4RsnoEwKPWbZg4Z6NUGaqP355SvtGWjjUFqEdmEGiFAB"
CALLER_SO="$REPO_ROOT/target/deploy/caller.so"
GEYSER_CONFIG="$PLUGIN_DIR/config/geyser-config.json"
LEDGER_DIR="$PLUGIN_DIR/.test-ledger"

echo ">> Building Anchor program (SBF)…"
( cd "$REPO_ROOT" && anchor build )

echo ">> Building Geyser plugin (native release dylib)…"
( cd "$PLUGIN_DIR" && cargo build --release )

# Reset the previous ledger so each run starts from a clean slate.
rm -rf "$LEDGER_DIR"

echo ">> Starting solana-test-validator with geyser plugin…"
echo "   tracked program: $PROGRAM_ID"
echo "   caller program:  $CALLER_ID (CPIs into the tracked program; not tracked)"
echo "   geyser config:   $GEYSER_CONFIG"
echo "   event log:       $PLUGIN_DIR/geyser-events.log"
echo

exec solana-test-validator \
  --reset \
  --ledger "$LEDGER_DIR" \
  --bpf-program "$PROGRAM_ID" "$PROGRAM_SO" \
  --bpf-program "$CALLER_ID" "$CALLER_SO" \
  --geyser-plugin-config "$GEYSER_CONFIG"
