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
    rm -rf "$E2E_DIR/node_modules/@fhevm/sdk"
    (cd "$E2E_DIR" && npm install --no-save "$TARBALL")
    ;;

  registry)
    VERSION="${2:-}"
    [[ -n "$VERSION" ]] || VERSION=$(node -p "require('$E2E_DIR/package.json').dependencies['@fhevm/sdk']")
    echo -e "${GREEN}install-sdk: installing @fhevm/sdk@${VERSION} from the npm registry...${NC}"
    rm -rf "$E2E_DIR/node_modules/@fhevm/sdk"
    (cd "$E2E_DIR" && npm install --no-save "@fhevm/sdk@${VERSION}")
    ;;

  *)
    echo -e "${RED}install-sdk: unknown mode '$MODE' (use 'local' or 'registry')${NC}" >&2
    exit 1
    ;;
esac

echo -e "${GREEN}install-sdk: done.${NC}"
