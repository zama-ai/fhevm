#!/usr/bin/env bash
set -euo pipefail

home_dir=$(realpath "$(dirname "$0")")
manual_packing_dirname="manual-packing"
pack_dir="$home_dir/$manual_packing_dirname"

GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}Cleaning existing tarball from $pack_dir${NC}"

mkdir -p "$pack_dir"

# Clean stale tarballs before packing
rm -f "$pack_dir"/fhevm-sdk-*.tgz

# Build
echo -e "${GREEN}Building project...${NC}"
(cd "$home_dir/../../.." && npm run build)

# Pack from src/ which holds the real package.json for distribution
echo -e "${GREEN}Packing project...${NC}"
(cd "$home_dir/../../../src" && npm pack --pack-destination "$pack_dir")

# Resolve the newly created tarball
tarball=$(echo "$pack_dir"/fhevm-sdk-*.tgz)
[[ -f "$tarball" ]] || { echo "Error: tarball not found in $pack_dir" >&2; exit 1; }

echo -e "${GREEN}Packed: $tarball${NC}"

# Update package.json to reference the correct tarball filename
echo -e "${GREEN}Updating package.json to reference the correct tarball filename${NC}"

tarball_name=$(basename "$tarball")
npm pkg set "dependencies.@fhevm/sdk=file:$manual_packing_dirname/$tarball_name" --prefix "$home_dir"

# Force reinstall: remove stale installation and lock file so npm re-extracts
# the new tarball instead of skipping it based on the cached integrity hash.
rm -rf "$home_dir/node_modules/@fhevm/sdk" "$home_dir/package-lock.json"

# Install and run the smoke test
echo -e "${GREEN}Installing...${NC}"
cd "$home_dir"
npm install

echo -e "${GREEN}Available test scripts:${NC}"
jq -r '.scripts | keys[] | "  npm run \(.)"' "$home_dir/package.json"
