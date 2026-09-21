#!/usr/bin/env bash
# demo-up.sh — bring up the confidential-vault demo stack (#1760), keep it running.
#
# Lifecycle-only glue invoked by `bun run demo up`; direct use is rejected so collision and ownership
# checks cannot be bypassed. It sequences existing steps:
#   1. bring the stack up from an ownership-checked empty state (clean-e2e.sh).
#   2. deploy the two demo programs (deploy-demo-programs.sh).
#   3. seed mints/vault/batchers/personas + write the demo-config JSON (bun demo:seed).
#   4. print the config path, operator URL, and status/log hints. NO teardown here — the stack is
#      meant to stay up for the dApp (#1761) / rehearsal (#1762). The e2e scenario suite is NEVER run here.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
FHEVM="$ROOT/test-suite/fhevm"
VALIDATOR_RPC="${SOLANA_RPC_URL:?missing lifecycle validator RPC URL}"

# The lifecycle hands every path down (layout under its FHEVM_STATE_DIR); this script computes none.
: "${FHEVM_STATE_DIR:?demo-up.sh is lifecycle-only; run 'bun run demo up' from the repository root}"
: "${DEMO_CONFIG_PATH:?missing lifecycle demo config path}"
: "${DEMO_MANIFEST_PATH:?missing lifecycle manifest path}"


: "${DEMO_LIFECYCLE_DIR:?demo-up.sh is lifecycle-only; run 'bun run demo up' from the repository root}"
: "${DEMO_BOOT_ID:?missing lifecycle boot identity}"
[ "${FHEVM_REFUSE_EXISTING:-}" = "1" ] || {
  echo "==> [demo-up] missing fail-closed lifecycle guard" >&2
  exit 1
}
: "${FHEVM_COMPOSE_PROJECT:?missing lifecycle Compose project}"
python3 - "$DEMO_MANIFEST_PATH" "$DEMO_BOOT_ID" "$DEMO_LIFECYCLE_DIR" "$ROOT" "$FHEVM_COMPOSE_PROJECT" <<'PY'
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

# The lifecycle starts Vite after this script returns. Install the confidential-token client's
# graph first: tsc follows `@fhevm/confidential-token` into that package's source. Then install
# the dapp graph once per fresh boot so local and CI bring-up never rely on Bun's implicit
# auto-install; reseed intentionally skips it.
if ! ( cd "$ROOT/solana/clients/confidential-token" && bun install --frozen-lockfile ); then
  echo "==> [demo-up] confidential-token client install failed; retrying once without cached registry data" >&2
  ( cd "$ROOT/solana/clients/confidential-token" && bun install --force --no-cache --frozen-lockfile )
fi
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
echo "    operator    : lifecycle-managed on ${DEMO_OPERATOR_URL:?missing lifecycle operator URL}"
echo "    smoke       : (cd $FHEVM && bun run demo:smoke)"
echo "    status      : (cd $ROOT && bun run demo status)"
echo "    logs        : (cd $ROOT && bun run demo logs)"
echo "    reseed      : (cd $ROOT && bun run demo reseed)"
echo "    down        : (cd $ROOT && bun run demo down)"
