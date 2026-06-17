#!/usr/bin/env bash
# Fully reproducible Solana e2e from a CLEAN fhevm-cli state (acceptance #2).
#
# One command brings up the WHOLE stack from scratch with the Solana code baked in
# (no hand-swapped containers), then the Solana side-stack, then drives the vertical.
#
# The kms-core image carrying `compute_link_solana` is pinned in the lock as
# CORE_VERSION=solana-ud-c57f52f-fix (kms-core is not an fhevm override group). The four
# source-built groups are passed as --override so they build from THIS worktree:
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
LOCK="$ROOT/.fhevm/state/locks/latest-main-feaf86e.json"

# 1. Pin the Solana-capable kms-core image in the lock (idempotent).
python3 - "$LOCK" <<'PY'
import json, sys
p = sys.argv[1]
d = json.load(open(p))
d["env"]["CORE_VERSION"] = "solana-ud-c57f52f-fix"
json.dump(d, open(p, "w"), indent=2)
print(f"[clean-e2e] pinned CORE_VERSION=solana-ud-c57f52f-fix in {p}")
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
    --override relayer \
    --override kms-connector \
    --allow-schema-mismatch )

# 3. Bring the Solana side-stack online against the freshly-deployed live backend.
#    Reads gateway addresses + KMS/coprocessor signer set live, so it tracks the new signer.
SKIP_BUILD=1 "$ROOT/solana/scripts/poc/setup-solana-side.sh"

echo "[clean-e2e] stack ready. Drive the vertical:"
echo "  - input/compute/public-decrypt/disclose/redeem : solana/scripts/poc/live-client"
echo "  - user-decrypt : SOLANA_UD_HANDLE=0x.. SOLANA_UD_EXPECTED=.. \\"
echo "      cargo test -p kms --features non-wasm --test solana_user_decrypt_live -- --ignored --nocapture"
