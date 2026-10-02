#!/usr/bin/env bash
# Runs the relayer on the host with cargo, logs in this terminal (Ctrl-C stops it). Usage: relayer-run.sh CONFIG
set -euo pipefail
CONFIG="${1:?config path}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
[ -f "$CONFIG" ] || { echo "no such config: $CONFIG (make config KMS=<n> T=<t> writes one)"; exit 1; }
echo "relayer-http with $CONFIG (KMS_API_KEY set)"
cd "$ROOT" && KMS_API_KEY="${API_KEY:-fhevm-e2e-kms-connector-api-key}" exec cargo run -p relayer-http -- "$CONFIG"
