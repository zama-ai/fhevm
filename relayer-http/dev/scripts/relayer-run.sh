#!/usr/bin/env bash
# Runs the relayer on the host with cargo, logs in this terminal (Ctrl-C stops it). Usage: relayer-run.sh CONFIG
# Signs with SIGNER_KEY (defaults to the e2e test key, KMS_CONNECTOR_ALLOWED_SENDER_PRIVATE_KEY in .env.test-suite,
# whose address proxies-up.sh allows), for the ProtocolConfig of the running stack. SIGNER_ADDRESS is the key's
# address (default: the e2e test key's): the relayer refuses to start when the two do not match.
set -euo pipefail
CONFIG="${1:?config path}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
STACK_ENV="$(cd "$ROOT/.." && pwd)/.fhevm/runtime/env/kms-connector.env"
[ -f "$CONFIG" ] || { echo "no such config: $CONFIG (make config KMS=<n> T=<t> writes one)"; exit 1; }
PROTOCOL_CONFIG="$(grep -s '^KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS=' "$STACK_ENV" | cut -d= -f2)"
[ -n "$PROTOCOL_CONFIG" ] || { echo "no ProtocolConfig address in $STACK_ENV: start the stack first (./fhevm-cli up)"; exit 1; }
echo "relayer-http with $CONFIG (ProtocolConfig $PROTOCOL_CONFIG, RELAYER_SIGNER_KEY set)"
cd "$ROOT" && \
  APP_KMS_AGGREGATOR__AUTH__PROTOCOL_CONFIG="$PROTOCOL_CONFIG" \
  APP_KMS_AGGREGATOR__AUTH__ADDRESS="${SIGNER_ADDRESS:-0xAE808c0968fF9557C8E0EAbcf57747c75B490Aa0}" \
  RELAYER_SIGNER_KEY="${SIGNER_KEY:-0xdcd7f0c4990a3df000d45f6eb2728a8cf93f0d16b1f6bdffbe07f5a250250e6c}" \
  exec cargo run -p relayer-http -- "$CONFIG"
