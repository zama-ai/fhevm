#!/usr/bin/env bash
# select-overrides.sh — decide which of the five source-built solana-e2e component groups must be
# built from this worktree (fhevm-cli --override) and which can run the branch-published images
# that solana-images-publish.yml pushes on every push to feature/solana.
#
# Usage (from CI, after `docker login ghcr.io`):
#   solana/scripts/e2e/select-overrides.sh <base-sha>
#     <base-sha>: full SHA of the commit the PR is applied onto (parent 1 of the pull_request
#                 merge commit that actions/checkout checks out).
#
# Selection rule:
#   1. Diff <base-sha>..HEAD and map changed paths to the five groups (see path map below).
#      Paths that affect the build recipe itself (this script, the e2e/publish workflows,
#      test-suite/fhevm, clean-e2e.sh) force building everything.
#   2. Groups whose image inputs changed are built from source (--override), exactly as before.
#   3. The untouched groups run the immutable base-commit images
#      `ghcr.io/zama-ai/fhevm/<image>:feature-solana-<base-sha>` (bit-for-bit the code the PR is
#      rebased onto) — but only if EVERY such image exists. If any is missing (first run ever,
#      publish still running, or the base commit was skipped by the publish queue), fall back to
#      building everything from source — never a broken run. The floating `:feature-solana` tag
#      is deliberately NOT consumed here: the publish matrix is fail-fast:false, so one failed
#      image job can leave that tag pointing at a previous push while sibling images advance,
#      and consuming it could silently mix components from different branch commits.
#
# Outputs (printed as key=value; appended to $GITHUB_OUTPUT when set):
#   overrides  space-separated group list to pass as --override, or "none" when every group
#              runs published images (SOLANA_E2E_OVERRIDES in clean-e2e.sh uses the same encoding)
#   lock_pins  space-separated KEY=TAG lock-env pins for the non-overridden groups (may be empty)
#
# Test seams:
#   CHANGED_FILES     newline-separated changed-file list; skips the git diff when set
#   MANIFEST_INSPECT  command probing image existence (default: docker manifest inspect)
set -euo pipefail

BASE_SHA="${1:?usage: select-overrides.sh <base-sha>}"
BASE_TAG="feature-solana-$BASE_SHA"

GROUPS_ALL=(gateway-contracts host-contracts coprocessor relayer kms-connector)

# Lock env keys per group (test-suite/fhevm/src/resolve/target.ts REPO_PACKAGES): pinning these in
# the lock makes the non-overridden compose services pull the branch image instead of the pinned
# main baseline. RELAYER_IMAGE_REPOSITORY is derived from RELAYER_VERSION by fhevm-cli (unparsed
# tags resolve to the modern ghcr.io/zama-ai/fhevm/relayer repository), so only versions are pinned.
lock_keys_for() {
  case "$1" in
    gateway-contracts) echo "GATEWAY_VERSION" ;;
    host-contracts) echo "HOST_VERSION" ;;
    coprocessor) echo "COPROCESSOR_DB_MIGRATION_VERSION COPROCESSOR_HOST_LISTENER_VERSION COPROCESSOR_GW_LISTENER_VERSION COPROCESSOR_TFHE_WORKER_VERSION COPROCESSOR_ZKPROOF_WORKER_VERSION COPROCESSOR_SNS_WORKER_VERSION COPROCESSOR_TX_SENDER_VERSION" ;;
    relayer) echo "RELAYER_VERSION RELAYER_MIGRATE_VERSION" ;;
    kms-connector) echo "CONNECTOR_DB_MIGRATION_VERSION CONNECTOR_GW_LISTENER_VERSION CONNECTOR_KMS_WORKER_VERSION CONNECTOR_TX_SENDER_VERSION" ;;
    *) echo "unknown group $1" >&2; return 1 ;;
  esac
}

# ghcr.io/zama-ai/<image> repositories per group; must stay in sync with solana-images-publish.yml.
images_for() {
  case "$1" in
    gateway-contracts) echo "fhevm/gateway-contracts" ;;
    host-contracts) echo "fhevm/host-contracts" ;;
    coprocessor) echo "fhevm/coprocessor/db-migration fhevm/coprocessor/host-listener fhevm/coprocessor/gw-listener fhevm/coprocessor/tfhe-worker fhevm/coprocessor/zkproof-worker fhevm/coprocessor/sns-worker fhevm/coprocessor/tx-sender" ;;
    relayer) echo "fhevm/relayer fhevm/relayer-migrate" ;;
    kms-connector) echo "fhevm/kms-connector/db-migration fhevm/kms-connector/gw-listener fhevm/kms-connector/kms-worker fhevm/kms-connector/tx-sender" ;;
    *) echo "unknown group $1" >&2; return 1 ;;
  esac
}

# Path -> groups map. Derived from the docker build inputs of each image:
#   - coprocessor (coprocessor/fhevm-engine/*/Dockerfile COPY lines — every engine image copies
#     the same workspace-wide input set): coprocessor/, listener/, gateway-contracts/,
#     host-contracts/, shared/ciphertext-attestation, the solana Rust tree (Cargo.toml + crates/
#     + programs/ + runtime-tests/), root package files. The solana crates/programs reach the
#     binaries through host-listener/Cargo.toml (zama-host, zama-solana-acl,
#     zama-solana-transaction) and tfhe-worker/Cargo.toml (confidential-token, zama-host,
#     zama-fhe).
#   - kms-connector (kms-connector/crates/*/Dockerfile + connector-db/Dockerfile COPY lines):
#     kms-connector/, both rust_bindings, shared/, and ONLY solana/crates/zama-solana-acl of the
#     solana tree (its Cargo.toml deliberately avoids workspace inheritance), hence the dedicated
#     zama-solana-acl rule below instead of the general solana fan-out.
#   - relayer (relayer/docker/relayer[-migrate]/Dockerfile bind mounts): relayer/, both
#     rust_bindings, shared/user-decryption-signature, whole solana Rust tree (zama-host +
#     transitive path deps via relayer/Cargo.toml).
#   - gateway-contracts / host-contracts images: their own trees + root package files.
# Non-Rust solana/ paths (scripts, geyser, docs, test-fixtures) are no longer copied into any
# builder stage, and runtime-tests sources are copied only for cargo workspace resolution and are
# never compiled into a runtime image, so none of them trigger a rebuild. Programs that no image
# consumes (demo-vault, confidential-deposit-app) are none-ruled explicitly; any FUTURE program
# falls into the general solana/programs/* rule and is over-built by default.
groups_for_path() {
  local path="$1"
  case "$path" in
    # Build-recipe changes: rebuild everything (a stale published image could mask the change).
    solana/scripts/e2e/select-overrides.sh|solana/scripts/e2e/clean-e2e.sh|.github/workflows/solana-e2e.yml|.github/workflows/solana-images-publish.yml|test-suite/fhevm/*|package.json|package-lock.json)
      echo "all" ;;
    coprocessor/*) echo "coprocessor" ;;
    listener/*) echo "coprocessor" ;;
    kms-connector/*) echo "kms-connector" ;;
    relayer/*) echo "relayer" ;;
    gateway-contracts/*) echo "gateway-contracts coprocessor kms-connector relayer" ;;
    host-contracts/*) echo "host-contracts coprocessor kms-connector relayer" ;;
    shared/*) echo "coprocessor kms-connector relayer" ;;
    # On-chain-only demo programs: no docker image compiles them (checked against the Cargo.tomls
    # and Dockerfile COPY lines cited above).
    solana/programs/demo-vault/*|solana/programs/confidential-deposit-app/*) echo "" ;;
    # The one solana crate the kms-connector images consume (kms-connector/crates/*/Dockerfile).
    solana/crates/zama-solana-acl/*) echo "coprocessor kms-connector relayer" ;;
    solana/programs/*|solana/crates/*|solana/Cargo.toml|solana/Cargo.lock) echo "coprocessor relayer" ;;
    *) echo "" ;;
  esac
}

if [ -z "${CHANGED_FILES+x}" ]; then
  # --no-renames is load-bearing: with rename detection, a pure move is listed only under its NEW
  # path, so moving a file OUT of a mapped tree (e.g. coprocessor/ -> sdk/) would change the image
  # contents without touching a mapped path and the stale published image would test false-green.
  # Disabling detection lists a move as delete(old)+add(new), so both sides hit the map.
  CHANGED_FILES="$(git diff --name-only --no-renames "$BASE_SHA" HEAD)"
fi

touched=""
build_all=false
while IFS= read -r file; do
  [ -n "$file" ] || continue
  groups="$(groups_for_path "$file")"
  if [ "$groups" = "all" ]; then
    echo "[select-overrides] $file changes the build recipe -> building all groups from source"
    build_all=true
    break
  fi
  for group in $groups; do
    case " $touched " in *" $group "*) ;; *) touched="$touched $group" ;; esac
  done
done <<< "$CHANGED_FILES"

untouched=""
if [ "$build_all" = true ]; then
  touched="${GROUPS_ALL[*]}"
else
  touched="${touched# }"
  for group in "${GROUPS_ALL[@]}"; do
    case " $touched " in *" $group "*) ;; *) untouched="$untouched $group" ;; esac
  done
  untouched="${untouched# }"
fi
echo "[select-overrides] source-changed groups: ${touched:-<none>}"
echo "[select-overrides] unchanged groups:      ${untouched:-<none>}"

manifest_exists() {
  ${MANIFEST_INSPECT:-docker manifest inspect} "$1" > /dev/null 2>&1
}

# All published images for the unchanged groups must exist at one tag; mixing tags across groups
# could pair components from different base commits.
all_images_exist_at() {
  local tag="$1" group image
  for group in $untouched; do
    for image in $(images_for "$group"); do
      if ! manifest_exists "ghcr.io/zama-ai/$image:$tag"; then
        echo "[select-overrides] missing ghcr.io/zama-ai/$image:$tag"
        return 1
      fi
    done
  done
}

selected_tag=""
if [ -n "$untouched" ]; then
  if all_images_exist_at "$BASE_TAG"; then
    selected_tag="$BASE_TAG"
    echo "[select-overrides] using base-commit images :$BASE_TAG for unchanged groups"
  else
    # No complete base-commit image set (first run ever, publish still running, or the base
    # commit was skipped by the publish queue). The floating :feature-solana tag is NOT a
    # fallback (see the header: a partial publish failure can leave it mixing branch commits),
    # so build everything from source — the pre-#1766 behavior.
    echo "[select-overrides] WARNING: no complete :$BASE_TAG image set published;" \
      "falling back to building all groups from source (pre-#1766 behavior)." \
      "Rerun once solana-images-publish.yml has finished for $BASE_SHA to get the skip."
    touched="${GROUPS_ALL[*]}"
    untouched=""
  fi
fi

lock_pins=""
for group in $untouched; do
  for key in $(lock_keys_for "$group"); do
    lock_pins="$lock_pins $key=$selected_tag"
  done
done
lock_pins="${lock_pins# }"

overrides="${touched:-none}"
echo "overrides=$overrides"
echo "lock_pins=$lock_pins"
if [ -n "${GITHUB_OUTPUT:-}" ]; then
  {
    echo "overrides=$overrides"
    echo "lock_pins=$lock_pins"
  } >> "$GITHUB_OUTPUT"
fi
