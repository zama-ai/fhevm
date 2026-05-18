#!/usr/bin/env bash
set -euo pipefail

# Installs one npm `tkms` release into the SDK wasm tree.
#
# 1. Normalize the requested version and resolve the `src/wasm/tkms/vX.Y.Z` destination.
# 2. Create a temporary npm project and install `tkms@X.Y.Z` or the source spec passed by `--source`.
# 3. Copy upstream `kms_lib.js`, `kms_lib.d.ts`, and `kms_lib_bg.wasm`.
# 4. Prettier-format `kms_lib.d.ts`.
# 5. Run `build-kms.ts` to generate SDK `kms_lib.js`, declarations, and type checks.
# 6. Run `wasm-to-base64.build.mjs` to generate `kms_lib_bg.wasm.base64.js` and `.d.ts`.

usage() {
  cat >&2 <<'EOF'
Usage:
  scripts/wasm/kms/install-tkms.sh <version> [--source <npm-spec>] [--no-compress] [--force|-y]

Installs tkms@<version> or <npm-spec> into a temporary npm project, then creates:
  src/wasm/tkms/v<version>/

The destination contains at least:
  kms_lib.js
  kms_lib.d.ts
  kms_lib_bg.wasm
  kms_lib_bg.wasm.base64.js
  kms_lib_bg.wasm.base64.d.ts
  type-check.test.ts

Example:
  scripts/wasm/kms/install-tkms.sh 0.13.10
  scripts/wasm/kms/install-tkms.sh v0.13.20-0 --no-compress --force
  scripts/wasm/kms/install-tkms.sh 0.13.20-0 --source file:../tkms/pkg --force

Environment:
  NODE=<node binary>  Override the Node.js binary.
  NPM=<npm binary>    Override the npm binary.
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ] || [ "$#" -eq 0 ]; then
  usage
  if [ "$#" -eq 0 ]; then
    exit 1
  fi
  exit 0
fi

version_arg=""
source_spec=""
force=0
no_compress=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --source)
      source_spec="${2:-}"
      [ -n "$source_spec" ] || die "missing value for $1"
      shift 2
      ;;
    --force|-y)
      force=1
      shift
      ;;
    --no-compress)
      no_compress=1
      shift
      ;;
    --*)
      die "unknown option: $1"
      ;;
    *)
      if [ -n "$version_arg" ]; then
        die "unexpected argument: $1"
      fi
      version_arg="$1"
      shift
      ;;
  esac
done

[ -n "$version_arg" ] || die "<version> is required, for example 0.13.10"

version="${version_arg#v}"
version_dir="v${version}"
install_spec="${source_spec:-tkms@${version}}"

script_dir="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
sdk_root="$(CDPATH= cd -- "${script_dir}/../../.." && pwd)"
dst_dir="${sdk_root}/src/wasm/tkms/${version_dir}"
build_script="${script_dir}/build-kms.ts"
wasm_to_base64_script="${script_dir}/../wasm-to-base64.build.mjs"

node_bin="${NODE:-node}"
npm_bin="${NPM:-npm}"

command -v "$node_bin" >/dev/null 2>&1 || die "Node.js binary not found: ${node_bin}"
command -v "$npm_bin" >/dev/null 2>&1 || die "npm binary not found: ${npm_bin}"
[ -f "$build_script" ] || die "missing build script: ${build_script}"
[ -f "$wasm_to_base64_script" ] || die "missing wasm-to-base64 script: ${wasm_to_base64_script}"

if [ -e "$dst_dir" ]; then
  if [ "$force" -eq 1 ]; then
    echo "[0/6] ${dst_dir} exists - overwriting (--force)"
  elif [ ! -t 0 ]; then
    die "${dst_dir} already exists and stdin is not a TTY. Re-run with --force."
  else
    printf "Destination %s already exists. Overwrite? [y/N] " "$dst_dir" >&2
    read -r answer
    case "$answer" in
      y|Y|yes|YES)
        ;;
      *)
        echo "Aborted."
        exit 0
        ;;
    esac
  fi
fi

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/tkms-install-XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

echo "[1/6] creating temporary npm project: ${tmp_dir}"
cat > "${tmp_dir}/package.json" <<'EOF'
{
  "name": "_tkms_installer",
  "version": "0.0.0",
  "private": true
}
EOF

echo "      installing ${install_spec}"
"$npm_bin" install --silent --no-fund --no-audit --no-save "$install_spec" --prefix "$tmp_dir"

src_pkg="${tmp_dir}/node_modules/tkms"
[ -d "$src_pkg" ] || die "tkms package not found at ${src_pkg} after installing ${install_spec}. Local package specs must resolve to a package named 'tkms'."
[ -f "${src_pkg}/kms_lib.js" ] || die "expected ${src_pkg}/kms_lib.js"
[ -f "${src_pkg}/kms_lib.d.ts" ] || die "expected ${src_pkg}/kms_lib.d.ts"
[ -f "${src_pkg}/kms_lib_bg.wasm" ] || die "expected ${src_pkg}/kms_lib_bg.wasm"

echo "[2/6] creating ${dst_dir}"
rm -rf "$dst_dir"
mkdir -p "$dst_dir"

cp "${src_pkg}/kms_lib.js" "${dst_dir}/kms_lib.js"
cp "${src_pkg}/kms_lib.d.ts" "${dst_dir}/kms_lib.d.ts"
cp "${src_pkg}/kms_lib_bg.wasm" "${dst_dir}/kms_lib_bg.wasm"

echo "[3/6] prettier-formatting ${dst_dir}/kms_lib.d.ts"
if ! (cd "$sdk_root" && npx --no-install prettier --ignore-path /dev/null --write "${dst_dir}/kms_lib.d.ts"); then
  echo "      prettier failed (continuing)" >&2
fi

echo "[4/6] generating SDK kms_lib.js, declarations, and type checks"
"$node_bin" "$build_script" "${dst_dir}/kms_lib.js" "${dst_dir}" --version "$version"

echo "[5/6] generating base64 wasm JS and d.ts"
wasm_to_base64_args=("${dst_dir}/kms_lib_bg.wasm" --export tkmsWasmBase64)
if [ "$no_compress" -eq 1 ]; then
  wasm_to_base64_args+=(--no-compress)
fi
"$node_bin" "$wasm_to_base64_script" "${wasm_to_base64_args[@]}"

echo "[6/6] done"
echo "      ${dst_dir}"
echo "      ${dst_dir}/kms_lib.js"
echo "      ${dst_dir}/kms_lib.d.ts"
echo "      ${dst_dir}/kms_lib_bg.wasm"
echo "      ${dst_dir}/kms_lib_bg.wasm.base64.js"
echo "      ${dst_dir}/kms_lib_bg.wasm.base64.d.ts"
if [ -f "${dst_dir}/type-check.test.ts" ]; then
  echo "      ${dst_dir}/type-check.test.ts"
fi
