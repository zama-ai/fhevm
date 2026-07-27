#!/usr/bin/env bash
set -euo pipefail

# Run the Playwright skeleton test from a clean slate: stop any leftover infra
# (down.sh), then run the test — which starts its own anvils + gateway, runs,
# and tears them down. A fully self-contained, clean run.
#
#   ./test/infra/run-skeleton.sh             # clean run
#   ./test/infra/run-skeleton.sh --headed    # extra args forwarded to playwright
#
# Requires foundry (anvil/cast/forge) on PATH.

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"$DIR/down.sh"

cd "$DIR/../browser-next"
exec npx playwright test specs/gw-skeleton.spec.ts --config playwright.config.ts "$@"
