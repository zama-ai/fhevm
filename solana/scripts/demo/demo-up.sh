#!/usr/bin/env bash
# demo-up.sh — bring up the confidential-vault demo stack (#1760), keep it running.
#
# Zero-logic glue only (repo policy: all real logic is TypeScript/bun). It sequences existing steps:
#   1. bring the stack up the e2e way (clean-e2e.sh) — UNLESS one is already running, in which case
#      it re-seeds against it (idempotent re-run, acceptance #6).
#   2. deploy the two demo programs (deploy-demo-programs.sh).
#   3. seed mints/vault/batchers/personas + write the demo-config JSON (bun demo:seed).
#   4. print the config path, faucet command, and status/log hints. NO teardown here — the stack is
#      meant to stay up for the dApp (#1761) / rehearsal (#1762). `full-vertical.sh` is NEVER run.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
FHEVM="$ROOT/test-suite/fhevm"
VALIDATOR_RPC="http://127.0.0.1:8899"

# Opt-in permissive CORS for the demo dApp origin (relayer defaults OFF). Export before bring-up so
# the relayer container inherits it; harmless on a re-run against an already-running stack.
export RELAYER_PERMISSIVE_CORS="${RELAYER_PERMISSIVE_CORS:-1}"

if curl -s -m2 "$VALIDATOR_RPC" -X POST -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' 2>/dev/null | grep -q '"ok"'; then
  echo "==> [demo-up] validator already healthy — re-seeding the running stack (idempotent)"
else
  echo "==> [demo-up] no running stack — bringing one up (clean-e2e.sh)"
  bash "$ROOT/solana/scripts/e2e/clean-e2e.sh"
fi

bash "$ROOT/solana/scripts/demo/deploy-demo-programs.sh"

( cd "$FHEVM" && bun run demo:seed )

CONFIG_PATH="$ROOT/.fhevm/runtime/solana-demo.json"
echo
echo "==> [demo-up] demo stack is up and seeded."
echo "    config JSON : $CONFIG_PATH"
echo "    faucet      : (cd $FHEVM && bun run demo:faucet)   # http://127.0.0.1:8090"
echo "    smoke       : (cd $FHEVM && bun run demo:smoke)"
echo "    status      : (cd $FHEVM && ./fhevm-cli status)"
echo "    logs        : /tmp/solana-*.log"
