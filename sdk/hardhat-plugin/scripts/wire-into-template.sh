#!/usr/bin/env bash
# Dev loop: place the built plugin PHYSICALLY into a Hardhat template's node_modules (not a symlink), so
# it resolves the template's single hardhat/ethers instance (a symlink would resolve this package's own
# devDep hardhat -> two-instance breakage of extendEnvironment). Also symlinks the cleartext package.
#
#   Usage:  ./scripts/wire-into-template.sh /path/to/fhevm-hardhat-template
#
# Then, in the template:
#   npx hardhat test test/FHECounter.ts                    # in-process hardhat network
#   anvil --silent & HARDHAT_NETWORK=anvil npx hardhat test test/FHECounter.ts   # external anvil
#     (the template's anvil accounts config is unfunded by default — anvil_setBalance the signers first)
set -euo pipefail

TEMPLATE="${1:?usage: wire-into-template.sh <template-dir>}"
PLUGIN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PKG_CLEARTEXT="$(cd "$PLUGIN_DIR/../../host-contracts-cleartext" && pwd)"

cd "$PLUGIN_DIR"
npm run build

DST="$TEMPLATE/node_modules/@fhevm/hardhat-plugin"
rm -rf "$DST"
mkdir -p "$DST"
cp "$PLUGIN_DIR/package.json" "$DST/"
cp -R "$PLUGIN_DIR/dist" "$DST/"

ln -sfn "$PKG_CLEARTEXT" "$TEMPLATE/node_modules/@fhevm/host-contracts-cleartext"

echo "wired @fhevm/hardhat-plugin (built) + @fhevm/host-contracts-cleartext into $TEMPLATE"
