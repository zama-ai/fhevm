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
#   - solana-proof-service : standalone MMR proof API (own override group; not piggybacked on relayer)
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
SOLANA_E2E_OVERRIDES="${SOLANA_E2E_OVERRIDES:-gateway-contracts host-contracts coprocessor relayer solana-proof-service kms-connector}"
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

# The local clients and demo import the public `@fhevm/sdk/solana` package exports, which resolve
# to the generated ESM tree. An implicit Bun install may have copied the file-linked SDK before that
# tree existed (including Bun's implicit install while entering `demo up`). Build the source, then
# force-refresh the frozen Bun graph so its package copy contains those generated exports and the
# checked-in CLI dependency versions. Assert the exact imports before paying for stack bring-up.
( cd "$ROOT/sdk/js-sdk" && npm ci && npm run build:esm )
( cd "$FHEVM" && bun install --force --frozen-lockfile )
( cd "$FHEVM" && bun -e "await import('@fhevm/sdk/solana'); await import('@fhevm/sdk/solana/vault')" )

# The source-built Rust services inherit their builder tag from each component's toolchain.
# Some tags are published for amd64 only. On an arm64 Docker daemon, keep those services native:
# reuse an exact local arm64 builder, let Docker pull a published arm64 manifest when available,
# or build the repository's canonical builder under the same tag once.
ensure_native_rust_builders() {
  local docker_arch
  docker_arch="$(docker info --format '{{.Architecture}}')"
  case "$docker_arch" in
    arm64 | aarch64) ;;
    *) return ;;
  esac

  local versions=""
  local group toolchain version image local_arch local_channel local_recipe
  local remote_manifest remote_has_arm temporary_image recipe_hash
  recipe_hash="$(git -C "$ROOT" hash-object golden-container-images/rust-glibc/Dockerfile)"
  for group in $SOLANA_E2E_OVERRIDES; do
    case "$group" in
      coprocessor) toolchain="$ROOT/coprocessor/fhevm-engine/rust-toolchain.toml" ;;
      kms-connector) toolchain="$ROOT/kms-connector/rust-toolchain.toml" ;;
      relayer) toolchain="$ROOT/relayer/rust-toolchain.toml" ;;
      *) continue ;;
    esac
    version="$(sed -n 's/^channel = "\([^"]*\)"/\1/p' "$toolchain")"
    if [ -z "$version" ]; then
      echo "[clean-e2e] unable to read Rust channel from $toolchain" >&2
      return 1
    fi
    case " $versions " in
      *" $version "*) ;;
      *) versions="${versions:+$versions }$version" ;;
    esac
  done

  for version in $versions; do
    image="ghcr.io/zama-ai/fhevm/gci/rust-glibc:$version"
    local_arch="$(docker image inspect --format '{{.Architecture}}' "$image" 2>/dev/null || true)"
    local_channel="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.channel"}}' "$image" 2>/dev/null || true)"
    local_recipe="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.recipe"}}' "$image" 2>/dev/null || true)"
    if [ "$local_arch" = "arm64" ] &&
      [ "$local_channel" = "$version" ] &&
      [ "$local_recipe" = "$recipe_hash" ]; then
      echo "[clean-e2e] reusing fingerprinted native arm64 Rust builder $image"
      continue
    fi

    if ! remote_manifest="$(docker buildx imagetools inspect --format '{{json .}}' "$image")"; then
      echo "[clean-e2e] unable to inspect the published platforms for $image" >&2
      return 1
    fi
    remote_has_arm="$(python3 - "$remote_manifest" <<'PY'
import json, sys

document = json.loads(sys.argv[1])

def contains_arm64(value):
    if isinstance(value, dict):
        architecture = value.get("architecture") or value.get("Architecture")
        os_name = value.get("os") or value.get("OS")
        if architecture in ("arm64", "aarch64") and os_name in (None, "linux"):
            return True
        return any(contains_arm64(child) for child in value.values())
    if isinstance(value, list):
        return any(contains_arm64(child) for child in value)
    return False

print("yes" if contains_arm64(document) else "no")
PY
)"
    if [ "$remote_has_arm" = "yes" ]; then
      echo "[clean-e2e] pulling published native arm64 Rust builder $image"
      docker pull --platform linux/arm64 "$image"
      local_arch="$(docker image inspect --format '{{.Architecture}}' "$image")"
      if [ "$local_arch" != "arm64" ]; then
        echo "[clean-e2e] expected published arm64 builder, got $local_arch for $image" >&2
        return 1
      fi
      continue
    fi

    temporary_image="fhevm-rust-glibc:$version-arm64-$$"
    echo "[clean-e2e] $image has no arm64 manifest; building its canonical image natively"
    docker build \
      --build-arg "RUST_IMAGE_VERSION=$version" \
      --label "org.zama.rust-glibc.channel=$version" \
      --label "org.zama.rust-glibc.recipe=$recipe_hash" \
      --tag "$temporary_image" \
      "$ROOT/golden-container-images/rust-glibc"
    local_arch="$(docker image inspect --format '{{.Architecture}}' "$temporary_image")"
    local_channel="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.channel"}}' "$temporary_image")"
    local_recipe="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.recipe"}}' "$temporary_image")"
    if [ "$local_arch" != "arm64" ] ||
      [ "$local_channel" != "$version" ] ||
      [ "$local_recipe" != "$recipe_hash" ]; then
      echo "[clean-e2e] native Rust builder validation failed for $temporary_image" >&2
      return 1
    fi
    docker image tag "$temporary_image" "$image"
    docker image rm "$temporary_image"
  done
}

ensure_native_rust_builders

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
#    fhevm-cli generates the Solana relayer + kms-connector config and boots solana-proof-service
#    (the solana-side bring-up below no longer patches those — single config writer).
#    `--override solana-proof-service` rebuilds the standalone proof image from this worktree so a
#    stale `solana-proof-service:local` / `:fhevm-local` image cannot outlive HEAD.
#    SOLANA_E2E_SCENARIO selects the fhevm-cli scenario. Default `solana` (centralized KMS).
#    Set to `solana-threshold-kms` to run the same vertical against a real 4-party threshold KMS
#    (fhevm-internal#1746); that scenario also requires KMS_THRESHOLD below so the on-chain
#    certificate thresholds match 2t+1 instead of the centralized default of 1.
( cd "$FHEVM" && ./fhevm-cli up \
    --scenario "$SOLANA_E2E_SCENARIO" \
    --lock-file "$LOCK" \
    ${OVERRIDE_ARGS[@]+"${OVERRIDE_ARGS[@]}"} \
    --allow-schema-mismatch )
# NOTE: relayer + kms-connector + solana-proof-service must run feature/solana
# worktree code (via --override), NOT the pinned 4f42734 baseline images: the
# prebuilt kms-connector at that tag rejects the generated Solana host_chains
# config ("missing field acl_address") — its config schema predates the
# optional-acl_address change the config generator (src/generate/solana.ts)
# assumes. Dropping these --overrides breaks clean-e2e unless
# SOLANA_E2E_LOCK_PINS points them at branch-published feature/solana images
# instead. `--override solana-proof-service` rebuilds the standalone proof
# image so a stale `:local` / `:fhevm-local` tag cannot outlive HEAD.

# 3. Bring the Solana side-stack online against the freshly-deployed live backend.
#    Reads gateway addresses + KMS/coprocessor signer set live, so it tracks the new signer.
#    The sole supported path deploys a reconstruction-first zama-host on the geyser-plugin validator and
#    ingests ordinary computation facts through Yellowstone reconstruction.
"$ROOT/solana/scripts/e2e/setup-solana-side.sh"

echo "[clean-e2e] stack ready. Drive the full vertical (input -> compute -> public/user-decrypt ->"
echo "  input-flow -> consume), user-decrypt is now PURE-SDK (no kms checkout):"
echo "    TE_VALUE=55 bash solana/scripts/e2e/full-vertical.sh"
