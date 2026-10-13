#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR=$(cd "$SCRIPT_DIR/../.." && pwd)
MANUAL_PACK_DIRNAME="manual-pack"
PACK_DIR="$SCRIPT_DIR/../$MANUAL_PACK_DIRNAME"

SKIP_BUILD=0
for arg in "$@"; do
  case "$arg" in
    --skip-build) SKIP_BUILD=1 ;;
    *)            echo "Error: unknown argument '$arg'. Use '--skip-build' to skip the build step." >&2; exit 1 ;;
  esac
done

GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Cleaning existing tarball from $PACK_DIR${NC}"

# Clean stale tarballs before packing
rm -f "$PACK_DIR"/fhevm-sdk-*.tgz

mkdir -p "$PACK_DIR"
PACK_DIR=$(cd "$PACK_DIR" && pwd)

# Build
if [[ "$SKIP_BUILD" -eq 1 ]]; then
  echo -e "${GREEN}Skipping build step.${NC}"
else
  echo -e "${GREEN}Building project...${NC}"
  (cd "$ROOT_DIR" && npm run build)
fi

# Pack from src/ which holds the real package.json for distribution
echo -e "${GREEN}Packing project...${NC}"
(cd "$ROOT_DIR/src" && npm pack --pack-destination "$PACK_DIR")

# Resolve the newly created tarball
TARBALL=$(echo "$PACK_DIR"/fhevm-sdk-*.tgz)
[[ -f "$TARBALL" ]] || { echo "Error: tarball not found in $PACK_DIR" >&2; exit 1; }

TARBALL_NAME=$(basename "$TARBALL")
echo -e "${GREEN}Packed: ${TARBALL_NAME}${NC}"

# The SDK's Solana entry needs its zama-host peer; sdk/js-sdk's postinstall already built its dist.
HOST_TARBALL_NAME=$(cd "$ROOT_DIR/../../solana/clients/zama-host" && npm pack --pack-destination "$PACK_DIR")

# Read peer dep versions from the root package.json
ETHERS_VERSION=$(node -p "require('$ROOT_DIR/package.json').devDependencies.ethers")
VIEM_VERSION=$(node -p "require('$ROOT_DIR/package.json').devDependencies.viem")
# The SDK's optional Solana peers, at the versions the SDK develops against.
SOLANA_PEERS=$(node -p "const versions = require('$ROOT_DIR/package.json').devDependencies; Object.keys(require('$ROOT_DIR/src/package.json').peerDependencies).filter((name) => /^@(solana|wallet-standard)\//.test(name)).map((name) => JSON.stringify(name) + ': ' + JSON.stringify(versions[name])).join(', ')")

# Write package.json referencing the freshly packed tarball
echo -e "${GREEN}Writing $MANUAL_PACK_DIRNAME/package.json...${NC}"
cat > "$PACK_DIR/package.json" <<EOF
{
  "name": "manual-pack-test",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "@fhevm/solana-zama-host": "file:${HOST_TARBALL_NAME}",
    ${SOLANA_PEERS},
    "@fhevm/sdk": "file:${TARBALL_NAME}"
  },
  "devDependencies": {
    "ethers": "${ETHERS_VERSION}",
    "viem": "${VIEM_VERSION}"
  }
}
EOF

# Force reinstall so npm re-extracts the new tarball
rm -rf "$PACK_DIR/node_modules" "$PACK_DIR/package-lock.json"

# Install the tarball into manual-pack/node_modules so vitest can resolve it
echo -e "${GREEN}Installing packed SDK into $MANUAL_PACK_DIRNAME/node_modules...${NC}"
(cd "$PACK_DIR" && npm install)

# The packed SDK's Solana entry loads as ESM and as CommonJS. Node's lookup would fall back to
# sdk/js-sdk/node_modules above this directory, so the SDK must also resolve zama-host and each
# Solana peer from this install.
(cd "$PACK_DIR" && node --input-type=module -e "await import('@fhevm/sdk/solana')" && node -e "
  const sdk = require.resolve('@fhevm/sdk/solana');
  require(sdk);
  const peers = Object.keys(require('@fhevm/sdk/package.json').peerDependencies);
  for (const name of peers.filter((name) => /^@(fhevm|solana|wallet-standard)\//.test(name))) {
    const found = require.resolve(name, { paths: [sdk] });
    if (!found.startsWith(process.cwd() + '/node_modules/')) throw new Error(name + ' resolved outside the pack: ' + found);
  }
")
