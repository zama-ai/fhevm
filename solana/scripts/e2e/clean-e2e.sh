#!/usr/bin/env bash
# clean-e2e.sh — bring up a clean local Solana + fhevm-cli vertical stack.
#
# Usage (from repo root):
#   bash solana/scripts/e2e/clean-e2e.sh
#
# When: before full-vertical / adversarial live runs; CI solana-e2e setup.
# Writes: local validator + Docker/fhevm-cli stack only (no checked-in goldens).
#
# Fully reproducible Solana e2e from a CLEAN fhevm-cli state (acceptance #2).
#
# One command brings up the WHOLE stack from scratch with the Solana code baked in
# (no hand-swapped containers), then the Solana side-stack, then drives the vertical.
#
# The kms-core image carrying `compute_link_solana` is pinned in the lock; its tag is the
# single source of truth in test-suite/fhevm/solana-images.env (kms-core is not an fhevm
# override group). The five source-built groups are passed as --override so they build from
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

# CI seam (#1766): which of the five source-built groups build from THIS worktree (--override),
# and optional KEY=TAG lock-env pins pointing the remaining groups at branch-published images
# (select-overrides.sh computes both in CI). Local runs keep the build-everything default; set
# SOLANA_E2E_OVERRIDES to "none" for an explicit empty override list.
SOLANA_E2E_OVERRIDES="${SOLANA_E2E_OVERRIDES:-gateway-contracts host-contracts coprocessor relayer kms-connector}"
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
#    fhevm-cli generates the Solana relayer + kms-connector config itself (the solana-side bring-up
#    below no longer patches those — single config writer).
( cd "$FHEVM" && ./fhevm-cli up \
    --scenario solana \
    --lock-file "$LOCK" \
    ${OVERRIDE_ARGS[@]+"${OVERRIDE_ARGS[@]}"} \
    --allow-schema-mismatch )
# NOTE: relayer + kms-connector must run feature/solana code, NOT the pinned 4f42734 baseline
# images: the prebuilt kms-connector at that tag rejects the generated Solana host_chains config
# ("missing field acl_address") — its config schema predates the optional-acl_address change the
# config generator (src/generate/solana.ts) assumes. Dropping these --overrides breaks clean-e2e
# unless SOLANA_E2E_LOCK_PINS points them at branch-published feature/solana images instead.

# 3. Bring the Solana side-stack online against the freshly-deployed live backend.
#    Reads gateway addresses + KMS/coprocessor signer set live, so it tracks the new signer.
#    The sole supported path deploys a reconstruction-first zama-host on the geyser-plugin validator and
#    ingests ordinary computation facts through Yellowstone reconstruction.
"$ROOT/solana/scripts/e2e/setup-solana-side.sh"

echo "[clean-e2e] stack ready. Drive the full vertical (input -> compute -> public/user-decrypt ->"
echo "  input-flow -> consume), user-decrypt is now PURE-SDK (no kms checkout):"
echo "    TE_VALUE=55 bash solana/scripts/e2e/full-vertical.sh"
