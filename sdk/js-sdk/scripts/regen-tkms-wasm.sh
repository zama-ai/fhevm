#!/usr/bin/env bash
# Build the TKMS WASM client from a kms commit and install it as the SDK's single TKMS version.
# EVM and Solana user decryption both load it through versionsManifest.js.
#
# DESIGNED TO BE DELETED: this exists only because Solana user decryption is not yet in a
# published `tkms` npm release; it lives on the kms `solana` branch. When kms publishes it,
# delete this script and .regen-tkms.env, set the published version in versionsManifest.js,
# src/core/runtime/WasmVersions-p.ts and scripts/wasm/loaders/KmsLibApi.template.d.ts, and run
# `npm run wasm:install -- --lib tkms --force` (it also regenerates the loaders). Also remove
# the "Skip the CDN smoke test on feature/solana" steps in js-sdk-dod.yml and js-sdk-tests.yml.
#
# Consumers (SDK, e2e, all CI) use the COMMITTED WASM and never run this. Only a maintainer
# bumping the client runs it. Requires the Rust toolchain + wasm-pack.
#
# kms path is an INPUT (no hardcoded dev path): optional path arg > $KMS_DIR >
# scripts/.regen-tkms.env > sibling default. The kms source is also required:
# positional source selector > $KMS_COMMIT > scripts/.regen-tkms.env.
# Pass an exact commit for reproducibility, or --feature-head to resolve the freshly fetched
# origin/$KMS_BRANCH head (default `solana`) to an exact commit before building.
# Usage:
#   sdk/js-sdk/scripts/regen-tkms-wasm.sh [/path/to/kms] <40-character-kms-commit|--feature-head>
#   sdk/js-sdk/scripts/regen-tkms-wasm.sh <40-character-kms-commit|--feature-head>
# Then set the printed version in versionsManifest.js, src/core/runtime/WasmVersions-p.ts and
# scripts/wasm/loaders/KmsLibApi.template.d.ts, and run `npm run codegen:loaders`.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ROOT="$(cd "$SDK_DIR/../.." && pwd)"
TKMS_DIR="$SDK_DIR/src/wasm/tkms"

# Optional .env (git-ignored) so colleagues pin their own kms checkout path / branch.
ENV_FILE="$SCRIPT_DIR/.regen-tkms.env"
# Preserve explicit environment inputs; the local file supplies defaults only.
KMS_DIR_INPUT="${KMS_DIR:-}"
KMS_BRANCH_INPUT="${KMS_BRANCH:-}"
KMS_COMMIT_INPUT="${KMS_COMMIT:-}"
# shellcheck source=/dev/null
[ -f "$ENV_FILE" ] && source "$ENV_FILE"

# Resolve the kms checkout: explicit path, then $KMS_DIR, then a sibling default. If the first
# argument is a source selector, the optional path was omitted.
FIRST_ARG="${1:-}"
SECOND_ARG="${2:-}"
SIBLING_KMS_DIR="$(cd "$ROOT/../../zama/kms" 2>/dev/null && pwd || true)"
KMS_DIR_DEFAULT="${KMS_DIR_INPUT:-${KMS_DIR:-$SIBLING_KMS_DIR}}"
KMS_BRANCH="${KMS_BRANCH_INPUT:-${KMS_BRANCH:-solana}}"
if [[ "$FIRST_ARG" = "--feature-head" || "$FIRST_ARG" =~ ^[0-9a-fA-F]{40}$ ]]; then
  if [ -n "$SECOND_ARG" ]; then
    echo "ERROR: pass the kms source either after the optional checkout path or as the only argument." >&2
    exit 1
  fi
  KMS_DIR="$KMS_DIR_DEFAULT"
  KMS_COMMIT="$FIRST_ARG"
else
  KMS_DIR="${FIRST_ARG:-$KMS_DIR_DEFAULT}"
  KMS_COMMIT="${SECOND_ARG:-${KMS_COMMIT_INPUT:-${KMS_COMMIT:-}}}"
fi

if [ -z "$KMS_DIR" ] || [ ! -d "$KMS_DIR" ]; then
  echo "ERROR: kms checkout not found. Pass it as the first arg, set \$KMS_DIR, or place it at ../../zama/kms relative to the repo root." >&2
  exit 1
fi
if [[ "$KMS_COMMIT" != "--feature-head" && ! "$KMS_COMMIT" =~ ^[0-9a-fA-F]{40}$ ]]; then
  echo "ERROR: pass an exact 40-character kms commit or --feature-head as the source argument, or set \$KMS_COMMIT." >&2
  exit 1
fi

command -v wasm-pack >/dev/null 2>&1 || {
  echo "ERROR: wasm-pack not found. Install: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh" >&2
  exit 1
}
[ -d "$KMS_DIR/core/service" ] || { echo "ERROR: '$KMS_DIR' is not a kms checkout (no core/service)." >&2; exit 1; }

# 1. Pin the kms source to an exact commit known to be on the freshly fetched provenance branch.
echo "[regen] kms checkout: $KMS_DIR  branch: $KMS_BRANCH  commit: $KMS_COMMIT"
git -C "$KMS_DIR" fetch --quiet origin \
  "+refs/heads/$KMS_BRANCH:refs/remotes/origin/$KMS_BRANCH" \
  "+refs/heads/main:refs/remotes/origin/main"
BRANCH_SHA="$(git -C "$KMS_DIR" rev-parse "refs/remotes/origin/$KMS_BRANCH")"
MAIN_SHA="$(git -C "$KMS_DIR" rev-parse "refs/remotes/origin/main")"
if [ "$KMS_COMMIT" = "--feature-head" ]; then
  HEAD_SHA="$BRANCH_SHA"
else
  if ! git -C "$KMS_DIR" cat-file -e "${KMS_COMMIT}^{commit}" 2>/dev/null; then
    echo "ERROR: kms commit $KMS_COMMIT is unavailable after fetching origin/$KMS_BRANCH." >&2
    exit 1
  fi
  HEAD_SHA="$(git -C "$KMS_DIR" rev-parse "${KMS_COMMIT}^{commit}")"
fi
if ! git -C "$KMS_DIR" merge-base --is-ancestor "$HEAD_SHA" "$BRANCH_SHA"; then
  echo "ERROR: kms commit $HEAD_SHA is not on fetched origin/$KMS_BRANCH ($BRANCH_SHA)." >&2
  exit 1
fi
# Record the common base against the main branch snapshot fetched with the provenance branch.
BASE_SHA="$(git -C "$KMS_DIR" merge-base "$HEAD_SHA" "$MAIN_SHA")"
KMS_VERSION="$(git -C "$KMS_DIR" show "${HEAD_SHA}:Cargo.toml" | awk '
  $0 == "[workspace.package]" { in_workspace_package = 1; next }
  in_workspace_package && /^\[/ { exit }
  in_workspace_package && $1 == "version" { gsub(/"/, "", $3); print $3; exit }
')"
if [[ ! "$KMS_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.-]+)?$ ]]; then
  echo "ERROR: could not derive a valid workspace version from kms commit $HEAD_SHA." >&2
  exit 1
fi
VERSION="${KMS_VERSION}-solana.${HEAD_SHA:0:8}"
echo "[regen] head=$HEAD_SHA base=$BASE_SHA -> version v$VERSION"

# 2. Build in an isolated worktree so the verified source cannot drift and the caller's
#    checkout is left untouched. This is the build kms releases as the `tkms` npm package.
BUILD_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/kms-tkms-regen.XXXXXX")"
BUILD_DIR="$BUILD_ROOT/kms"
cleanup() {
  git -C "$KMS_DIR" worktree remove --force "$BUILD_DIR" >/dev/null 2>&1 || true
  rm -rf "$BUILD_ROOT"
}
trap cleanup EXIT
git -C "$KMS_DIR" worktree add --detach --quiet "$BUILD_DIR" "$HEAD_SHA"
( cd "$BUILD_DIR/core/service" && wasm-pack build --target web --out-dir pkg-web . --no-default-features )
PKG="$BUILD_DIR/core/service/pkg-web"
[ -f "$PKG/kms_lib_bg.wasm" ] || { echo "ERROR: wasm-pack did not produce $PKG/kms_lib_bg.wasm" >&2; exit 1; }
# kms publishes this package as `tkms`, and install-tkms.sh requires that name.
node -e 'const fs = require("fs"); const p = process.argv[1]; const j = JSON.parse(fs.readFileSync(p)); j.name = "tkms"; fs.writeFileSync(p, JSON.stringify(j, null, 2) + "\n");' "$PKG/package.json"

# 3. Install it as the only TKMS version, the same way as a published release.
find "$TKMS_DIR" -mindepth 1 -maxdepth 1 -type d -name 'v*' -exec rm -rf {} +
"$SCRIPT_DIR/wasm/kms/install-tkms.sh" "$VERSION" --source "file:$PKG" --force

# 4. Stamp provenance so the committed WASM is traceable to an exact kms commit (the branch moves).
cat > "$TKMS_DIR/KMS_BUILT_FROM" <<EOF
# Provenance of the vendored TKMS WASM (regen-tkms-wasm.sh). Do not hand-edit.
kms_branch=$KMS_BRANCH
kms_head=$HEAD_SHA
kms_base=$BASE_SHA
tkms_wasm_version=v$VERSION
EOF

echo "[regen] installed src/wasm/tkms/v$VERSION. Set $VERSION as the TKMS version (see the header),"
echo "[regen] run \`npm run codegen:loaders\`, then commit src/wasm/tkms/."
