#!/usr/bin/env bash
set -euo pipefail

# Installs one npm `tfhe` release into the SDK wasm tree.
#
# 1. Normalize the requested version and resolve the `src/wasm/tfhe/vX.Y.Z` destination.
# 2. Create a temporary npm project and install `tfhe@X.Y.Z` or the source spec passed by `--source`.
# 3. Copy upstream `tfhe.js`, `tfhe.d.ts`, `tfhe_bg.wasm`, and wasm declarations when present.
# 4. Run `build-tfhe.ts` to generate SDK `tfhe.js`, `tfhe-worker.mjs`, `startWorkers.js`, and type checks.
# 5. Run `wasm-to-base64.build.mjs` to generate `tfhe_bg.wasm.base64.js` and `.d.ts`.

usage() {
  cat >&2 <<'EOF'
Usage:
  scripts/wasm/tfhe/install-tfhe.sh <version> [--source <npm-spec>] [--force|-y]

Installs tfhe@<version> or <npm-spec> into a temporary npm project, then creates:
  src/wasm/tfhe/v<version>/

The destination contains at least:
  tfhe.js
  tfhe.d.ts
  tfhe_bg.wasm
  tfhe_bg.wasm.base64.js
  tfhe_bg.wasm.base64.d.ts
  tfhe-worker.mjs
  startWorkers.js
  type-check.test.ts

Example:
  scripts/wasm/tfhe/install-tfhe.sh 1.6.0
  scripts/wasm/tfhe/install-tfhe.sh v1.6.0 --force
  scripts/wasm/tfhe/install-tfhe.sh 1.6.0 --source file:./src/wasm/tfhe/dev.local --force

Environment:
  NODE=<node binary>  Override the Node.js binary.
  NPM=<npm binary>    Override the npm binary.
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

copy_if_exists() {
  local src="$1"
  local dst="$2"

  if [ -e "$src" ]; then
    cp -R "$src" "$dst"
  fi
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

[ -n "$version_arg" ] || die "<version> is required, for example 1.6.0"

version="${version_arg#v}"
version_dir="v${version}"
install_spec="${source_spec:-tfhe@${version}}"

script_dir="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
sdk_root="$(CDPATH= cd -- "${script_dir}/../../.." && pwd)"
dst_dir="${sdk_root}/src/wasm/tfhe/${version_dir}"
build_script="${script_dir}/build-tfhe.ts"
wasm_to_base64_script="${script_dir}/../wasm-to-base64.build.mjs"

node_bin="${NODE:-node}"
npm_bin="${NPM:-npm}"

command -v "$node_bin" >/dev/null 2>&1 || die "Node.js binary not found: ${node_bin}"
command -v "$npm_bin" >/dev/null 2>&1 || die "npm binary not found: ${npm_bin}"
[ -f "$build_script" ] || die "missing build script: ${build_script}"
[ -f "$wasm_to_base64_script" ] || die "missing wasm-to-base64 script: ${wasm_to_base64_script}"

if [ -e "$dst_dir" ]; then
  if [ "$force" -eq 1 ]; then
    echo "[0/5] ${dst_dir} exists - overwriting (--force)"
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

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/tfhe-install-XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

echo "[1/5] creating temporary npm project: ${tmp_dir}"
cat > "${tmp_dir}/package.json" <<'EOF'
{
  "name": "_tfhe_installer",
  "version": "0.0.0",
  "private": true
}
EOF

echo "      installing ${install_spec}"
"$npm_bin" install --silent --no-fund --no-audit --no-save "$install_spec" --prefix "$tmp_dir"

src_pkg="${tmp_dir}/node_modules/tfhe"
[ -d "$src_pkg" ] || die "tfhe package not found at ${src_pkg} after installing ${install_spec}. Local package specs must resolve to a package named 'tfhe'."
[ -f "${src_pkg}/tfhe.js" ] || die "expected ${src_pkg}/tfhe.js"
[ -f "${src_pkg}/tfhe_bg.wasm" ] || die "expected ${src_pkg}/tfhe_bg.wasm"

echo "[2/5] creating ${dst_dir}"
rm -rf "$dst_dir"
mkdir -p "$dst_dir"

cp "${src_pkg}/tfhe.js" "${dst_dir}/tfhe.js"
cp "${src_pkg}/tfhe_bg.wasm" "${dst_dir}/tfhe_bg.wasm"

copy_if_exists "${src_pkg}/tfhe.d.ts" "${dst_dir}/tfhe.d.ts"
copy_if_exists "${src_pkg}/tfhe_bg.wasm.d.ts" "${dst_dir}/tfhe_bg.wasm.d.ts"

echo "[3/5] generating SDK tfhe.js, worker, startWorkers.js, and type checks"
"$node_bin" "$build_script" "${dst_dir}/tfhe.js" "${dst_dir}" --version "$version"

echo "[4/5] generating base64 wasm JS and d.ts"
"$node_bin" "$wasm_to_base64_script" "${dst_dir}/tfhe_bg.wasm" --export tfheWasmBase64

echo "[5/5] done"
echo "      ${dst_dir}"
echo "      ${dst_dir}/tfhe.js"
if [ -f "${dst_dir}/tfhe.d.ts" ]; then
  echo "      ${dst_dir}/tfhe.d.ts"
fi
echo "      ${dst_dir}/tfhe_bg.wasm"
if [ -f "${dst_dir}/tfhe_bg.wasm.d.ts" ]; then
  echo "      ${dst_dir}/tfhe_bg.wasm.d.ts"
fi
echo "      ${dst_dir}/tfhe_bg.wasm.base64.js"
echo "      ${dst_dir}/tfhe_bg.wasm.base64.d.ts"
echo "      ${dst_dir}/tfhe-worker.mjs"
echo "      ${dst_dir}/startWorkers.js"
if [ -f "${dst_dir}/type-check.test.ts" ]; then
  echo "      ${dst_dir}/type-check.test.ts"
fi
