#!/usr/bin/env bash
# Fully reproducible Solana e2e from a CLEAN fhevm-cli state (acceptance #2).
#
# One command brings up the WHOLE stack from scratch with the Solana code baked in
# (no hand-swapped containers), then the Solana side-stack, then drives the vertical.
#
# The kms-core image carrying `compute_link_solana` is pinned in the lock; its tag is the
# single source of truth in test-suite/fhevm/solana-images.env (kms-core is not an fhevm
# override group). The four source-built groups are passed as --override so they build from
# THIS worktree:
#   - gateway-contracts : userDecryptionRequestSolana + verifyProofRequestSolana
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
# Pin the EVM stack to the main SHA this PoC was validated against. RFC-021 / Solana host support
# is not yet on a release bundle, so we resolve a specific main commit explicitly.
BASE_SHA="feaf86e"
LOCK="$ROOT/.fhevm/state/locks/sha-$BASE_SHA.json"

# 0. Resolve the pinned bundle so the lock exists even from a fully clean state (fhevm-cli clean
#    removes .fhevm). Idempotent.
( cd "$FHEVM" && ./fhevm-cli resolve --target sha --sha "$BASE_SHA" )

# 1. Pin the Solana-capable kms-core image in the lock (idempotent).
#    CORE_VERSION comes from the single source of truth so it cannot drift from the TS call sites.
# shellcheck source=/dev/null
source "$FHEVM/solana-images.env"
python3 - "$LOCK" "$CORE_VERSION" <<'PY'
import json, sys
p, core = sys.argv[1], sys.argv[2]
d = json.load(open(p))
d["env"]["CORE_VERSION"] = core
json.dump(d, open(p, "w"), indent=2)
print(f"[clean-e2e] pinned CORE_VERSION={core} in {p}")
PY

# 2. Clean rebuild of the whole EVM stack with the Solana code baked in from bootstrap.
#    The `solana` scenario declares the RFC-021 Solana host alongside the default EVM host, so
#    fhevm-cli generates the Solana relayer + kms-connector config itself (the solana-side bring-up
#    below no longer patches those — single config writer).
( cd "$FHEVM" && ./fhevm-cli up \
    --scenario solana \
    --lock-file "$LOCK" \
    --override gateway-contracts \
    --override coprocessor \
    --allow-schema-mismatch )
# Only coprocessor (host-listener) carries this branch's changes, so only it must build from
# source. relayer/ and kms-connector/ are byte-identical to the pinned solana-images.env images
# (RELAYER_VERSION / CONNECTOR_*_VERSION = 4f42734), so we drop their --override and pull the
# prebuilt images instead of rebuilding (~10-14 min/run). The kms-worker host.docker.internal
# mapping is a compose-level extra_hosts, so it still applies to the prebuilt image.

# 3. Bring the Solana side-stack online against the freshly-deployed live backend.
#    Reads gateway addresses + KMS/coprocessor signer set live, so it tracks the new signer.
#    RECONSTRUCT=1 (adjacent CI run) deploys an emitless zama-host on the geyser-plugin
#    validator and ingests via gRPC reconstruction; it force-builds emitless, so SKIP_BUILD
#    is moot there. Default (unset) keeps the SKIP_BUILD=1 native/emit path unchanged.
RECONSTRUCT="${RECONSTRUCT:-0}" SKIP_BUILD=1 "$ROOT/solana/scripts/poc/setup-solana-side.sh"

echo "[clean-e2e] stack ready. Drive the full vertical (input -> compute -> public/user-decrypt ->"
echo "  input-flow -> consume), user-decrypt is now PURE-SDK (no kms checkout):"
echo "    TE_VALUE=55 bash solana/scripts/poc/full-vertical.sh"
