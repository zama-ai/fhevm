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

# The source-built Rust services inherit their builder tag from each component's toolchain.
# Some tags are published for amd64 only. On arm64, build the canonical image once under a
# fingerprinted cache tag, alias it only while Compose builds, then restore the prior local tag.
NATIVE_RUST_BUILDER_ALIASES=()
NATIVE_RUST_BUILDER_BACKUPS=()
NATIVE_RUST_BUILDER_IMAGE_IDS=()
NATIVE_RUST_BUILDER_PREEXISTED=()

cleanup_native_rust_builder_aliases() {
  local original_status=$?
  local cleanup_failed=0
  local index image backup_image expected_id preexisted current_id
  for ((index = 0; index < ${#NATIVE_RUST_BUILDER_ALIASES[@]}; index++)); do
    image="${NATIVE_RUST_BUILDER_ALIASES[$index]}"
    backup_image="${NATIVE_RUST_BUILDER_BACKUPS[$index]}"
    expected_id="${NATIVE_RUST_BUILDER_IMAGE_IDS[$index]}"
    preexisted="${NATIVE_RUST_BUILDER_PREEXISTED[$index]}"
    current_id="$(docker image inspect --format '{{.Id}}' "$image" 2>/dev/null || true)"
    if [ "$current_id" != "$expected_id" ]; then
      echo "[clean-e2e] not restoring $image because its image changed after the temporary alias was installed" >&2
      cleanup_failed=1
      continue
    fi
    if [ -n "$backup_image" ]; then
      if ! docker image tag "$backup_image" "$image"; then
        echo "[clean-e2e] failed to restore previous tag $image from $backup_image" >&2
        cleanup_failed=1
        continue
      fi
      if ! docker image rm "$backup_image" >/dev/null; then
        echo "[clean-e2e] restored $image but failed to remove $backup_image" >&2
        cleanup_failed=1
        continue
      fi
      echo "[clean-e2e] restored previous local tag $image"
    elif [ "$preexisted" = "yes" ]; then
      echo "[clean-e2e] preserved pre-existing native tag $image"
    elif ! docker image rm "$image" >/dev/null; then
      echo "[clean-e2e] failed to remove temporary alias $image" >&2
      cleanup_failed=1
    else
      echo "[clean-e2e] removed temporary alias $image"
    fi
  done
  NATIVE_RUST_BUILDER_ALIASES=()
  NATIVE_RUST_BUILDER_BACKUPS=()
  NATIVE_RUST_BUILDER_IMAGE_IDS=()
  NATIVE_RUST_BUILDER_PREEXISTED=()
  if [ "$original_status" -ne 0 ]; then
    return "$original_status"
  fi
  return "$cleanup_failed"
}

install_native_rust_builder_alias() {
  local image="$1"
  local cache_image="$2"
  local backup_image="$3"
  local cache_id="$4"
  local preexisted="$5"
  if ! docker image tag "$cache_image" "$image"; then
    if [ -n "$backup_image" ] && ! docker image rm "$backup_image" >/dev/null; then
      echo "[clean-e2e] failed to install $image and remove its unused backup $backup_image" >&2
    fi
    return 1
  fi
  NATIVE_RUST_BUILDER_ALIASES+=("$image")
  NATIVE_RUST_BUILDER_BACKUPS+=("$backup_image")
  NATIVE_RUST_BUILDER_IMAGE_IDS+=("$cache_id")
  NATIVE_RUST_BUILDER_PREEXISTED+=("$preexisted")
}

ensure_native_rust_builders() {
  local docker_arch
  docker_arch="$(docker info --format '{{.Architecture}}')"
  case "$docker_arch" in
    arm64 | aarch64) ;;
    *) return ;;
  esac

  local versions=""
  local group toolchain version image cache_image backup_image preexisted
  local local_arch local_channel local_recipe local_id cache_id
  local remote_manifest remote_has_arm recipe_hash recipe_short
  recipe_hash="$(git -C "$ROOT" hash-object golden-container-images/rust-glibc/Dockerfile)"
  recipe_short="${recipe_hash:0:12}"
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
    if ! remote_manifest="$(docker buildx imagetools inspect --format '{{json .}}' "$image")"; then
      echo "[clean-e2e] unable to inspect published platforms for $image" >&2
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
        echo "[clean-e2e] expected arm64, got $local_arch for $image" >&2
        return 1
      fi
      continue
    fi

    cache_image="fhevm-rust-glibc-local:$version-arm64-$recipe_short"
    local_arch="$(docker image inspect --format '{{.Architecture}}' "$cache_image" 2>/dev/null || true)"
    local_channel="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.channel"}}' "$cache_image" 2>/dev/null || true)"
    local_recipe="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.recipe"}}' "$cache_image" 2>/dev/null || true)"
    if [ "$local_arch" != "arm64" ] ||
      [ "$local_channel" != "$version" ] ||
      [ "$local_recipe" != "$recipe_hash" ]; then
      echo "[clean-e2e] $image has no arm64 manifest; building its canonical image natively"
      docker build \
        --build-arg "RUST_IMAGE_VERSION=$version" \
        --label "org.zama.rust-glibc.channel=$version" \
        --label "org.zama.rust-glibc.recipe=$recipe_hash" \
        --tag "$cache_image" \
        "$ROOT/golden-container-images/rust-glibc"
    else
      echo "[clean-e2e] reusing fingerprinted native arm64 Rust builder $cache_image"
    fi

    local_arch="$(docker image inspect --format '{{.Architecture}}' "$cache_image")"
    local_channel="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.channel"}}' "$cache_image")"
    local_recipe="$(docker image inspect --format '{{index .Config.Labels "org.zama.rust-glibc.recipe"}}' "$cache_image")"
    if [ "$local_arch" != "arm64" ] ||
      [ "$local_channel" != "$version" ] ||
      [ "$local_recipe" != "$recipe_hash" ]; then
      echo "[clean-e2e] native Rust builder validation failed for $cache_image" >&2
      return 1
    fi

    cache_id="$(docker image inspect --format '{{.Id}}' "$cache_image")"
    local_id="$(docker image inspect --format '{{.Id}}' "$image" 2>/dev/null || true)"
    backup_image=""
    preexisted="no"
    if [ -n "$local_id" ]; then
      preexisted="yes"
    fi
    if [ -n "$local_id" ] && [ "$local_id" != "$cache_id" ]; then
      backup_image="fhevm-rust-glibc-restore:$version-$$"
      docker image tag "$image" "$backup_image"
    fi
    install_native_rust_builder_alias "$image" "$cache_image" "$backup_image" "$cache_id" "$preexisted"
    echo "[clean-e2e] installed temporary native alias $image"
  done
}

if [ "${SOLANA_E2E_LIBRARY_ONLY:-0}" = "1" ]; then
  # shellcheck disable=SC2317 # `return` is for sourced tests; `exit` is for direct execution.
  return 0 2>/dev/null || exit 0
fi

# The local clients and demo import the public `@fhevm/sdk/solana` package exports. Each
# consumer's postinstall replaces bun's `file:` snapshot with a symlink to the live source tree,
# so `node_modules/@fhevm/sdk` always serves the current build: install the SDK build workspace
# (the root graph its runtime dependencies resolve from), then generate the ESM and declaration
# trees the symlink serves. Rebuilds are visible to consumers immediately — nothing re-copies a
# snapshot.
( cd "$ROOT" && npm ci --workspace=@fhevm/sdk-dev --workspace=@fhevm/sdk --include-workspace-root=false )
( cd "$ROOT/sdk/js-sdk" && npm run clean && npm run build:esm && npm run build:types )
( cd "$FHEVM" && bun install --frozen-lockfile )
[ -L "$FHEVM/node_modules/@fhevm/sdk" ]
# Prove both runtimes resolve the SDK and its dependencies through the symlink.
( cd "$FHEVM" && node --input-type=module -e "await import('@fhevm/sdk/solana')" )
( cd "$FHEVM" && bun -e "await import('@fhevm/sdk/solana')" )

trap cleanup_native_rust_builder_aliases EXIT
ensure_native_rust_builders

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
#    fhevm-cli generates the Solana relayer + kms-connector config and boots solana-proof-service
#    (the Solana host-process step does not patch those — single config writer).
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
cleanup_native_rust_builder_aliases
trap - EXIT
# `up`'s host-process step cargo-builds host-listener, whose build.rs runs
# `npm ci` inside host-contracts and reifies the workspace root to that graph
# alone — wiping the SDK install above. Restore it before Vite / e2e / seed
# resolve `@fhevm/sdk` through the symlink.
( cd "$ROOT" && npm ci --workspace=@fhevm/sdk-dev --workspace=@fhevm/sdk --include-workspace-root=false )
( cd "$FHEVM" && node --input-type=module -e "await import('@fhevm/sdk/solana')" )
( cd "$FHEVM" && bun -e "await import('@fhevm/sdk/solana')" )
# NOTE: relayer + kms-connector + solana-proof-service must run feature/solana
# worktree code (via --override), NOT the pinned 4f42734 baseline images: the
# prebuilt kms-connector at that tag rejects the generated Solana host_chains
# config ("missing field acl_address") — its config schema predates the
# optional-acl_address change the config generator (src/generate/solana.ts)
# assumes. Dropping these --overrides breaks clean-e2e unless
# SOLANA_E2E_LOCK_PINS points them at branch-published feature/solana images
# instead. `--override solana-proof-service` rebuilds the standalone proof
# image so a stale `:local` / `:fhevm-local` tag cannot outlive HEAD.

# 3. The Solana side-stack (fresh geyser validator + program deploy, the typed zama-host bootstrap,
#    host-chain registration, the host-listener) is no longer a separate call: the `solana` scenario
#    resolves its host chain to `nodeProvisioning: host-process`, so the `up` above ran it as the
#    `host-process` pipeline step. `bun run src/solana/deploy.ts` still provisions the same thing
#    standalone when a node needs rebuilding without a full stack cycle.

echo "[clean-e2e] stack ready. Run the typed scenario suite (compute -> public/user-decrypt ->"
echo "  input-flow -> transfer -> consume), user-decrypt is PURE-SDK (no kms checkout):"
echo "    cd test-suite/fhevm && bun run test:e2e"
