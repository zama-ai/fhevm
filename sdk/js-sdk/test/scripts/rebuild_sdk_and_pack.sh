#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR=$(cd "$SCRIPT_DIR/../.." && pwd)
MANUAL_PACK_DIRNAME="manual-pack"
PACK_DIR="$SCRIPT_DIR/../$MANUAL_PACK_DIRNAME"

GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Cleaning existing tarball from $PACK_DIR${NC}"

# Clean stale tarballs before packing
rm -f "$PACK_DIR"/fhevm-sdk-*.tgz

mkdir -p "$PACK_DIR"
PACK_DIR=$(cd "$PACK_DIR" && pwd)

# Build
echo -e "${GREEN}Building project...${NC}"
(cd "$ROOT_DIR" && npm run build)

# Pack from src/ which holds the real package.json for distribution
echo -e "${GREEN}Packing project...${NC}"
(cd "$ROOT_DIR/src" && npm pack --pack-destination "$PACK_DIR")

# Resolve the newly created tarball
TARBALL=$(echo "$PACK_DIR"/fhevm-sdk-*.tgz)
[[ -f "$TARBALL" ]] || { echo "Error: tarball not found in $PACK_DIR" >&2; exit 1; }

TARBALL_NAME=$(basename "$TARBALL")
echo -e "${GREEN}Packed: ${TARBALL_NAME}${NC}"

# Read peer dep versions from the root package.json
ETHERS_VERSION=$(node -p "require('$ROOT_DIR/package.json').devDependencies.ethers")
VIEM_VERSION=$(node -p "require('$ROOT_DIR/package.json').devDependencies.viem")

# Write package.json referencing the freshly packed tarball
echo -e "${GREEN}Writing $MANUAL_PACK_DIRNAME/package.json...${NC}"
cat > "$PACK_DIR/package.json" <<EOF
{
  "name": "manual-pack-test",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
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
