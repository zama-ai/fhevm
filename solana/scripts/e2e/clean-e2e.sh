#!/usr/bin/env bash
# clean-e2e.sh — bring up a clean local Solana + fhevm-cli vertical stack.
#
# Usage (from repo root):
#   bash solana/scripts/e2e/clean-e2e.sh
#
# When: before running the live scenario suite (`bun run test:e2e`); CI solana-e2e setup.
# Writes: local validator + Docker/fhevm-cli stack only (no checked-in goldens).
#
# Fully reproducible Solana e2e from a CLEAN fhevm-cli state (acceptance #2).
#
# One command brings up the WHOLE stack from scratch with the Solana code baked in
# (no hand-swapped containers), then the Solana side-stack, then drives the vertical.
#
# The kms-core image carrying `compute_link_solana` is pinned in the lock; its tag is the
# single source of truth in test-suite/fhevm/solana-images.env (kms-core is not an fhevm
# override group). The six source-built groups are passed as --override so they build from
# THIS worktree (by default — CI narrows the set via SOLANA_E2E_OVERRIDES/SOLANA_E2E_LOCK_PINS,
# substituting branch-published images for groups the PR does not touch, see select-overrides.sh):
#   - gateway-contracts : userDecryptionRequestSolana + verifyProofRequestSolana
#   - host-contracts    : must track HEAD because the source-built kms-connector's gw-listener
#                         reads ProtocolConfig.getCurrentKmsContextAndEpoch() at startup (the
#                         epoch-lifecycle interface, #2615). The pinned baseline predates it, so a
#                         stock host-sc image lacks the method and the startup context-store reverts.
#   - coprocessor       : FULL group from this worktree (zkproof-worker 128B aux, tx-sender
#                         Solana EIP-712, plus host-listener/sns/tfhe + db-migration) so the
#                         DB schema and ALL coprocessor binaries are one consistent version
#                         (a per-service subset leaves stock services expecting newer columns)
#   - relayer           : bytes32 host identity, Solana user-decrypt calldata + ed25519 seam
#   - kms-connector     : Solana user-decrypt vertical (gw-listener + kms-worker)
#
# Because `kms-signer` discovers the kms-core's ACTUAL signer and registers it on-chain,
# and `bootstrap` triggers keygen into THAT kms-core, the trust model is consistent by
# construction -- the failure mode of hand-swapping the kms-core (signer + FHE key drift)
# cannot occur. MAINNET-safe: validator pinned to 127.0.0.1:8899.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
FHEVM="$ROOT/test-suite/fhevm"

# CI seam (#1766): which of the six source-built groups build from THIS worktree (--override),
# and optional KEY=TAG lock-env pins pointing the remaining groups at branch-published images
# (select-overrides.sh computes both in CI). Local runs keep the build-everything default; set
# SOLANA_E2E_OVERRIDES to "none" for an explicit empty override list.
SOLANA_E2E_OVERRIDES="${SOLANA_E2E_OVERRIDES:-gateway-contracts host-contracts coprocessor relayer kms-connector}"
# Scenario + KMS corruption threshold t. Defaults reproduce the centralized PoC exactly.
# `solana-threshold-kms` + KMS_THRESHOLD=1 runs the 4-party (3t+1) threshold KMS.
SOLANA_E2E_SCENARIO="${SOLANA_E2E_SCENARIO:-solana}"
export KMS_THRESHOLD="${KMS_THRESHOLD:-0}"
SOLANA_E2E_LOCK_PINS="${SOLANA_E2E_LOCK_PINS:-}"
if [ "$SOLANA_E2E_OVERRIDES" = "none" ]; then
  SOLANA_E2E_OVERRIDES=""
fi
OVERRIDE_ARGS=()
for group in $SOLANA_E2E_OVERRIDES; do
  OVERRIDE_ARGS+=(--override "$group")
done
echo "[clean-e2e] source-built overrides: ${SOLANA_E2E_OVERRIDES:-<none>}"
if [ -n "$SOLANA_E2E_LOCK_PINS" ]; then
  echo "[clean-e2e] lock pins for published images: $SOLANA_E2E_LOCK_PINS"
fi

# The local clients and demo import the public `@fhevm/sdk/solana` package exports. Each
# consumer's postinstall replaces bun's `file:` snapshot with a symlink to the live source tree,
# so `node_modules/@fhevm/sdk` serves the current build: install the SDK's own dependency
# graph, then generate the ESM and declaration
# trees the symlink serves. Rebuilds are visible to consumers immediately — nothing re-copies a
# snapshot.
( cd "$ROOT/sdk/js-sdk" && npm ci )
( cd "$ROOT/sdk/js-sdk" && npm run clean && npm run build:esm && npm run build:types )
( cd "$ROOT/solana/deploy" && bun install --frozen-lockfile )
( cd "$ROOT/solana/clients/confidential-token" && bun install --frozen-lockfile )
( cd "$FHEVM" && bun install --frozen-lockfile )
[ -L "$FHEVM/node_modules/@fhevm/sdk" ]
# Prove both runtimes resolve the SDK and its dependencies through the symlink.
( cd "$FHEVM" && node --input-type=module -e "await import('@fhevm/sdk/solana')" )
( cd "$FHEVM" && bun -e "await import('@fhevm/sdk/solana')" )

# The real Squads v4 program for the delegated-decrypt scenario, fetched from mainnet and
# sha256-pinned (nothing is committed — see the fetch script's header). Offline is non-fatal by
# default: the stack still boots and only the Squads scenario refuses to run. A PIN MISMATCH
# (exit 2) is fatal: the upstream program changed, and nothing may run against an unreviewed
# binary. SOLANA_E2E_REQUIRE_SQUADS=1 makes an unavailable fixture fatal too — the CI lane sets
# it, because a green run that silently skipped the only real 2-of-3 multisig arc proves less
# than it appears to. An offline laptop leaves it unset and keeps the skip.
squads_fixtures_status=0
bash "$ROOT/solana/scripts/e2e/fetch-squads-fixtures.sh" || squads_fixtures_status=$?
if [ "$squads_fixtures_status" -eq 2 ]; then
  echo "[clean-e2e] Squads fixture PIN MISMATCH — review the upstream change (see above) before running e2e" >&2
  exit 1
elif [ "$squads_fixtures_status" -ne 0 ]; then
  if [ "${SOLANA_E2E_REQUIRE_SQUADS:-}" = "1" ]; then
    echo "[clean-e2e] Squads fixtures unavailable and SOLANA_E2E_REQUIRE_SQUADS=1 — the Squads delegation scenario is required in this lane" >&2
    exit 1
  fi
  echo "[clean-e2e] WARN: Squads fixtures unavailable; the Squads delegation scenario will not run"
fi

# Pin the EVM stack to the main SHA this PoC was validated against. RFC-021 / Solana host support
# is not yet on a release bundle, so we resolve a specific main commit explicitly.
BASE_SHA="feaf86e"
LOCK="$ROOT/.fhevm/state/locks/sha-$BASE_SHA.json"

# 0. Resolve the pinned bundle so the lock exists even from a fully clean state (fhevm-cli clean
#    removes .fhevm). Idempotent.
( cd "$FHEVM" && ./fhevm-cli resolve --target sha --sha "$BASE_SHA" )

# 1. Pin the Solana-capable kms-core image in the lock (idempotent).
#    CORE_VERSION comes from the single source of truth so it cannot drift from the TS call sites.
#    SOLANA_E2E_LOCK_PINS additionally repoints non-overridden groups at branch-published image
#    tags (space-separated KEY=TAG entries, see select-overrides.sh).
# shellcheck source=/dev/null
source "$FHEVM/solana-images.env"
# shellcheck disable=SC2086 # SOLANA_E2E_LOCK_PINS is a space-separated KEY=TAG list, one arg each
python3 - "$LOCK" "CORE_VERSION=$CORE_VERSION" $SOLANA_E2E_LOCK_PINS <<'PY'
import json, sys
p = sys.argv[1]
d = json.load(open(p))
for pin in sys.argv[2:]:
    key, _, tag = pin.partition("=")
    d["env"][key] = tag
    print(f"[clean-e2e] pinned {key}={tag} in {p}")
json.dump(d, open(p, "w"), indent=2)
PY

# 2. Clean rebuild of the whole EVM stack with the Solana code baked in from bootstrap.
#    The `solana` scenario declares the RFC-021 Solana host alongside the default EVM host, so
#    fhevm-cli generates the Solana relayer + kms-connector config (the Solana host-process step
#    does not patch those — single config writer).
#    SOLANA_E2E_SCENARIO selects the fhevm-cli scenario. Default `solana` (centralized KMS).
#    Set to `solana-threshold-kms` to run the same vertical against a real 4-party threshold KMS
#    (fhevm-internal#1746); that scenario also requires KMS_THRESHOLD below so the on-chain
#    certificate thresholds match 2t+1 instead of the centralized default of 1.
( cd "$FHEVM" && ./fhevm-cli up \
    --scenario "$SOLANA_E2E_SCENARIO" \
    --lock-file "$LOCK" \
    ${OVERRIDE_ARGS[@]+"${OVERRIDE_ARGS[@]}"} \
    --allow-schema-mismatch )
( cd "$FHEVM" && node --input-type=module -e "await import('@fhevm/sdk/solana')" )
( cd "$FHEVM" && bun -e "await import('@fhevm/sdk/solana')" )
# NOTE: relayer + kms-connector run the worktree code (via --override). SOLANA_E2E_LOCK_PINS can
# point them at the branch-published `feature-solana-<sha>` images instead (see select-overrides.sh).

# 3. The Solana side-stack (fresh geyser validator + program deploy, the typed zama-host bootstrap,
#    host-chain registration, the host-listener) is no longer a separate call: the `solana` scenario
#    resolves its host chain to `nodeProvisioning: host-process`, so the `up` above ran it as the
#    `host-process` pipeline step. `bun run src/solana/deploy.ts` still provisions the same thing
#    standalone when a node needs rebuilding without a full stack cycle.

echo "[clean-e2e] stack ready. Run the typed scenario suite (compute -> public/user-decrypt ->"
echo "  input-flow -> transfer -> consume), user-decrypt is PURE-SDK (no kms checkout):"
echo "    cd test-suite/fhevm && bun run test:e2e"
