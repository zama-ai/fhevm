#!/usr/bin/env bash
# demo-up.sh — bring up the confidential-vault demo stack (#1760), keep it running.
#
# Lifecycle-only glue invoked by `bun run demo up`; direct use is rejected so collision and ownership
# checks cannot be bypassed. It sequences existing steps:
#   1. bring the stack up from an ownership-checked empty state (clean-e2e.sh).
#   2. deploy the two demo programs (deploy-demo-programs.sh).
#   3. seed mints/vault/batchers/personas + write the demo-config JSON (bun demo:seed).
#   4. print the config path, faucet command, and status/log hints. NO teardown here — the stack is
#      meant to stay up for the dApp (#1761) / rehearsal (#1762). `full-vertical.sh` is NEVER run.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
FHEVM="$ROOT/test-suite/fhevm"
VALIDATOR_RPC="http://127.0.0.1:8899"

# Absolute demo-config path, exported so the seed (runs from $FHEVM) and every later consumer resolve
# the SAME file regardless of their working directory. demo/config.ts honors DEMO_CONFIG_PATH; without
# this, the seed would write a CWD-relative path under $FHEVM while this script advertises a repo-root
# path — the mismatch the unified DEMO_CONFIG_PATH contract closes.
export DEMO_CONFIG_PATH="${DEMO_CONFIG_PATH:-$ROOT/.fhevm/runtime/solana-demo.json}"

# Opt-in permissive CORS for the demo dApp origin (relayer + proof service default OFF). Exported
# before bring-up so the relayer and solana-proof-service compose services pick these up via their
# ${..._PERMISSIVE_CORS:-} passthroughs (see docker-compose/relayer-docker-compose.yml and
# solana-proof-service-docker-compose.yml) and both containers come up with the demo CORS layer on;
# harmless on a re-run against an already-running stack.
export RELAYER_PERMISSIVE_CORS="${RELAYER_PERMISSIVE_CORS:-1}"
export SOLANA_PROOF_PERMISSIVE_CORS="${SOLANA_PROOF_PERMISSIVE_CORS:-1}"

: "${DEMO_LIFECYCLE_DIR:?demo-up.sh is lifecycle-only; run 'bun run demo up' from the repository root}"
: "${DEMO_BOOT_ID:?missing lifecycle boot identity}"
[ "${FHEVM_REFUSE_EXISTING:-}" = "1" ] || {
  echo "==> [demo-up] missing fail-closed lifecycle guard" >&2
  exit 1
}
: "${FHEVM_COMPOSE_PROJECT:?missing lifecycle Compose project}"
python3 - "$ROOT/.fhevm/runtime/solana-demo/manifest.json" "$DEMO_BOOT_ID" "$DEMO_LIFECYCLE_DIR" "$ROOT" "$FHEVM_COMPOSE_PROJECT" <<'PY'
import json
import os
import sys

manifest_path, boot_id, runtime_dir, repo_root, compose_project = sys.argv[1:]
with open(manifest_path, encoding="utf-8") as handle:
    manifest = json.load(handle)
if (
    manifest.get("bootId") != boot_id
    or manifest.get("state") != "starting"
    or manifest.get("repoRoot") != repo_root
    or manifest.get("composeProject") != compose_project
    or os.path.basename(runtime_dir) != boot_id
):
    raise SystemExit("demo-up.sh refused invalid lifecycle ownership context")
PY
if curl -s -m2 "$VALIDATOR_RPC" -X POST -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' 2>/dev/null | grep -q '"ok"'; then
  echo "==> [demo-up] refusing to attach lifecycle boot to an existing validator" >&2
  exit 1
fi
echo "==> [demo-up] lifecycle-owned fresh bring-up (clean-e2e.sh)"
bash "$ROOT/solana/scripts/e2e/clean-e2e.sh"

# The lifecycle starts Vite after this script returns. Install its frozen graph once per fresh boot
# so local and CI bring-up never rely on Bun's implicit auto-install; reseed intentionally skips it.
if ! ( cd "$ROOT/solana/demo-dapp" && bun install --frozen-lockfile ); then
  echo "==> [demo-up] dependency install failed; retrying once without cached registry data" >&2
  ( cd "$ROOT/solana/demo-dapp" && bun install --force --no-cache --frozen-lockfile )
fi

bash "$ROOT/solana/scripts/demo/deploy-demo-programs.sh"

# Bun resolves the SDK's file-linked build artifacts at their physical source path, outside this
# package's node_modules tree. Point its fallback lookup at the frozen demo graph so the SDK's own
# runtime dependencies resolve without requiring an unrelated repository-root install.
( cd "$FHEVM" && NODE_PATH="$ROOT/solana/demo-dapp/node_modules" bun run demo:seed )

CONFIG_PATH="$DEMO_CONFIG_PATH"
echo
echo "==> [demo-up] demo stack is up and seeded."
echo "    config JSON : $CONFIG_PATH"
echo "    faucet      : lifecycle-managed on http://127.0.0.1:8090"
echo "    smoke       : (cd $FHEVM && bun run demo:smoke)"
echo "    status      : (cd $ROOT && bun run demo status)"
echo "    logs        : (cd $ROOT && bun run demo logs)"
echo "    reseed      : (cd $ROOT && bun run demo reseed)"
echo "    down        : (cd $ROOT && bun run demo down)"
