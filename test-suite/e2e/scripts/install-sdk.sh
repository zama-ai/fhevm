#!/usr/bin/env bash
# Switches test-suite/e2e's installed @fhevm/sdk between a local pack of
# sdk/js-sdk/src and a fixed version from the npm registry, without touching
# package.json/package-lock.json — mirrors sdk/js-sdk/test/browser-next's
# refresh-sdk.sh `npm install --no-save` technique.
set -euo pipefail

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

E2E_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK_DIR="$(cd "$E2E_DIR/../../sdk/js-sdk" && pwd)"
ROOT_DIR="$(cd "$E2E_DIR/../.." && pwd)"

MODE="${1:-local}"

case "$MODE" in
  local)
    BUILD_PROFILE="${SDK_BUILD_PROFILE:-prod}"
    echo -e "${GREEN}install-sdk: building and packing sdk/js-sdk (profile: $BUILD_PROFILE)...${NC}"
    "$SDK_DIR/test/scripts/rebuild_sdk_and_pack.sh" "--build-profile=$BUILD_PROFILE"

    TARBALL=$(echo "$SDK_DIR"/test/manual-pack/fhevm-sdk-*.tgz)
    [[ -f "$TARBALL" ]] || {
      echo -e "${RED}install-sdk: no tarball found in $SDK_DIR/test/manual-pack${NC}" >&2
      exit 1
    }

    echo -e "${GREEN}install-sdk: installing $(basename "$TARBALL") into test-suite/e2e...${NC}"
    rm -rf "$E2E_DIR/node_modules/@fhevm/sdk" "$ROOT_DIR/node_modules/@fhevm/sdk"
    # --no-workspaces: without it, npm hoists @fhevm/sdk to the repo root node_modules
    # since nothing there conflicts with it, but ethers (its optional peer dep) stays
    # nested under test-suite/e2e/node_modules (pinned there by other workspace members'
    # own ethers requirements) — leaving the hoisted copy unable to resolve 'ethers'.
    # --legacy-peer-deps: this install resolves the tree from scratch (unlike the
    # root's `npm ci`, which trusts the checked-in lockfile), and @safe-global/safe-contracts'
    # peer dep on ethers@5.4.0 conflicts with the rest of the project's ethers@^6.15.0.
    (cd "$E2E_DIR" && npm install --no-save --no-workspaces --legacy-peer-deps "$TARBALL")
    ;;

  registry)
    VERSION="${2:-}"
    # package.json intentionally doesn't pin @fhevm/sdk (it's installed here, not
    # declared as a manifest dependency), so there's no default version to fall
    # back to — the caller must always pass one explicitly.
    [[ -n "$VERSION" ]] || {
      echo -e "${RED}install-sdk: registry mode requires a version, e.g. './install-sdk.sh registry 0.13.2'${NC}" >&2
      exit 1
    }
    echo -e "${GREEN}install-sdk: installing @fhevm/sdk@${VERSION} from the npm registry...${NC}"
    rm -rf "$E2E_DIR/node_modules/@fhevm/sdk" "$ROOT_DIR/node_modules/@fhevm/sdk"
    # --legacy-peer-deps: see comment in the 'local' branch above.
    (cd "$E2E_DIR" && npm install --no-save --no-workspaces --legacy-peer-deps "@fhevm/sdk@${VERSION}")
    ;;

  *)
    echo -e "${RED}install-sdk: unknown mode '$MODE' (use 'local' or 'registry')${NC}" >&2
    exit 1
    ;;
esac

# The --no-workspaces install above resolves test-suite/e2e's tree standalone,
# which shadows @fhevm/solidity's workspace link (-> ../../../library-solidity)
# with a real copy installed from the registry — silently compiling e2e against
# a different, possibly stale, @fhevm/solidity than the one in this repo. Put
# the workspace link back.
echo -e "${GREEN}install-sdk: restoring @fhevm/solidity workspace link...${NC}"
rm -rf "$E2E_DIR/node_modules/@fhevm/solidity"
ln -s "$ROOT_DIR/library-solidity" "$E2E_DIR/node_modules/@fhevm/solidity"

echo -e "${GREEN}install-sdk: done.${NC}"
