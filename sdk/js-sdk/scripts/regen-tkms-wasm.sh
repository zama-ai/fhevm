#!/usr/bin/env bash
# Regenerate the vendored TKMS WASM bindings from a kms checkout. SINGLE producer of
# sdk/js-sdk/src/wasm/tkms/kms_lib*. See FI#1546.
#
# DESIGNED TO BE DELETED: this exists only because the Solana de-signcryption
# (process_user_decryption_resp_solana / compute_link_solana) is not yet in a published
# @zama-fhe TKMS package — it lives on the kms `feature/solana` branch. When kms ships those
# bits in a real release, delete this script + .regen-tkms.env + the vendored blob and
# `npm i` the published package instead.
#
# Consumers (SDK, e2e, all CI) use the COMMITTED blob and never run this. Only a maintainer
# bumping the bindings runs it. Requires the Rust toolchain + wasm-pack.
#
# kms path is an INPUT (no hardcoded dev path):  arg $1  >  $KMS_DIR  >  scripts/.regen-tkms.env  >  sibling default
# Usage:  sdk/js-sdk/scripts/regen-tkms-wasm.sh [/path/to/kms]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ROOT="$(cd "$SDK_DIR/../.." && pwd)"
TKMS_DIR="$SDK_DIR/src/wasm/tkms"

# Optional .env (git-ignored) so colleagues pin their own kms checkout path / branch.
ENV_FILE="$SCRIPT_DIR/.regen-tkms.env"
# shellcheck source=/dev/null
[ -f "$ENV_FILE" ] && source "$ENV_FILE"

# Resolve the kms checkout: explicit arg, then $KMS_DIR, then a sibling default.
KMS_DIR="${1:-${KMS_DIR:-$(cd "$ROOT/../../zama/kms" 2>/dev/null && pwd || echo /Users/work/code/zama/kms)}}"
KMS_BRANCH="${KMS_BRANCH:-feature/solana}"

command -v wasm-pack >/dev/null 2>&1 || {
  echo "ERROR: wasm-pack not found. Install: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh" >&2
  exit 1
}
[ -d "$KMS_DIR/core/service" ] || { echo "ERROR: '$KMS_DIR' is not a kms checkout (no core/service)." >&2; exit 1; }

# 1. Pin the kms source: track the branch (ergonomics), record the exact (base,head) for provenance.
echo "[regen] kms checkout: $KMS_DIR  branch: $KMS_BRANCH"
git -C "$KMS_DIR" fetch --quiet origin "$KMS_BRANCH" 2>/dev/null || true
git -C "$KMS_DIR" checkout --quiet "$KMS_BRANCH"
HEAD_SHA="$(git -C "$KMS_DIR" rev-parse HEAD)"
# The branch is "pinned on" an upstream kms base; record it if the branch name encodes it, else best-effort.
BASE_SHA="$(git -C "$KMS_DIR" merge-base HEAD origin/main 2>/dev/null || echo unknown)"
VERSION="${TKMS_WASM_VERSION:-solana-${HEAD_SHA:0:8}}"
echo "[regen] head=$HEAD_SHA base=$BASE_SHA -> version v$VERSION"

# 2. Build the wasm bindings exactly as kms CI does (npm-release.yml / wasm-testing.yml).
( cd "$KMS_DIR/core/service" && wasm-pack build --target web . --no-default-features )
PKG="$KMS_DIR/core/service/pkg"
[ -f "$PKG/kms_lib_bg.wasm" ] || { echo "ERROR: wasm-pack did not produce $PKG/kms_lib_bg.wasm" >&2; exit 1; }

# 3. Vendor the blob under versioned names (the SDK imports version-embedded filenames).
cp "$PKG/kms_lib.js"        "$TKMS_DIR/kms_lib.v$VERSION.js"
cp "$PKG/kms_lib.d.ts"      "$TKMS_DIR/kms_lib.v$VERSION.d.ts"
cp "$PKG/kms_lib_bg.wasm"   "$TKMS_DIR/kms_lib_bg.v$VERSION.wasm"

# 4. Emit the base64-inlined copy (bundlers lazy-load this ~0.85MB chunk; no fetch of the .wasm).
B64_JS="$TKMS_DIR/kms_lib_bg.v$VERSION.wasm.base64.js"
node - "$TKMS_DIR/kms_lib_bg.v$VERSION.wasm" "$B64_JS" <<'NODE'
const fs = require("node:fs"), crypto = require("node:crypto");
const [wasmPath, outPath] = process.argv.slice(2);
const b64 = fs.readFileSync(wasmPath).toString("base64");
const sha = crypto.createHash("sha256").update(b64).digest("hex"); // hash of the base64 payload
fs.writeFileSync(outPath,
  `// Auto-generated — do not edit. Run: sdk/js-sdk/scripts/regen-tkms-wasm.sh\n` +
  `// SHA-256: ${sha}\n` +
  `export const tkmsWasmBase64 = "${b64}";\n`);
NODE
printf 'export const tkmsWasmBase64: string;\n' > "$TKMS_DIR/kms_lib_bg.v$VERSION.wasm.base64.d.ts"

# 5. Stamp provenance so the committed blob is traceable to an exact kms commit (the branch moves).
cat > "$TKMS_DIR/KMS_BUILT_FROM" <<EOF
# Provenance of the vendored TKMS WASM (regen-tkms-wasm.sh). Do not hand-edit.
kms_branch=$KMS_BRANCH
kms_head=$HEAD_SHA
kms_base=$BASE_SHA
tkms_wasm_version=v$VERSION
EOF

# 6. Rewire the SDK import sites from the previously-vendored version to the new one.
OLD="$(ls "$TKMS_DIR"/kms_lib.v*.js 2>/dev/null | sed -E 's#.*/kms_lib\.v(.*)\.js#\1#' | grep -vx "$VERSION" | head -1 || true)"
if [ -n "$OLD" ]; then
  echo "[regen] rewiring imports v$OLD -> v$VERSION in decrypt/module/{init,api}-p.ts"
  for f in "$SDK_DIR/src/core/modules/decrypt/module/init-p.ts" "$SDK_DIR/src/core/modules/decrypt/module/api-p.ts"; do
    sed -i.bak "s/v$OLD/v$VERSION/g" "$f" && rm -f "$f.bak"
  done
  echo "[regen] NOTE: remove the old kms_lib.v$OLD.* blob in a separate step once the new build is verified."
fi

echo "[regen] done. Verify with the SDK decrypt tests, then commit the v$VERSION blob + KMS_BUILT_FROM."
