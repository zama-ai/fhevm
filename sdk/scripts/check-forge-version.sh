#!/usr/bin/env bash
#
# Asserts that the installed forge matches the version declared in the calling package's
# ./.foundry-version.
#
# Usage:  ./scripts/check-forge-version.sh [PACKAGE_ROOT]
#
#   PACKAGE_ROOT   directory holding .foundry-version. Defaults to the current directory, which is
#                  what npm sets to the package root when it runs a script — so package.json needs
#                  only:  "check:forge-version": "\"$(npm prefix)/scripts/check-forge-version.sh\""
#
# SHARED SCRIPT. Lives at sdk/scripts/ and is used by every workspace member that pins a forge
# version. It is deliberately generic: it knows nothing about any particular package, only that the
# caller declares a version and that `forge --version` must match it. Keep it that way — anything
# package-specific belongs in that package's own scripts/.
#
# `sdk/scripts/` is NOT a workspace member and needs no package.json: shell scripts are invoked by
# path, so they need neither node module resolution nor bin symlinks. `$(npm prefix)` resolves to the
# workspace root from any member at any depth, which is what makes the invocation above portable.
#
# Why a pin is needed at all: `forge fmt` output can change between forge releases. Where a package
# stores generated or normalised Solidity, a forge upgrade would make every such file non-compliant
# at once, and the failure would surface as N mysterious content errors rather than as "you changed
# formatter versions". Run this before those checks so the real cause is named.
#
# To move a pin deliberately:
#   foundryup --install <version>
#   echo "<version>" > .foundry-version      # in the package being moved
#   forge fmt                                # re-normalise that package's Solidity
#
# Exits non-zero on mismatch, on a missing or empty declaration, or if forge is not installed.
set -euo pipefail

PACKAGE_ROOT="${1:-$PWD}"
cd "$PACKAGE_ROOT"

PIN_FILE=".foundry-version"

if [ ! -f "$PIN_FILE" ]; then
    echo "Error: $PIN_FILE not found in $PACKAGE_ROOT." >&2
    exit 1
fi

EXPECTED="$(tr -d '[:space:]' < "$PIN_FILE")"
if [ -z "$EXPECTED" ]; then
    echo "Error: $PACKAGE_ROOT/$PIN_FILE is empty." >&2
    exit 1
fi

if ! command -v forge > /dev/null 2>&1; then
    echo "Error: forge is not installed or not on PATH (expected $EXPECTED)." >&2
    exit 1
fi

ACTUAL="$(forge --version 2> /dev/null | head -1 | sed 's/^forge Version: //')"

if [ "$ACTUAL" != "$EXPECTED" ]; then
    cat >&2 << EOF
Error: forge version mismatch.

  package:                 $PACKAGE_ROOT
  expected (${PIN_FILE}):  $EXPECTED
  installed:               $ACTUAL

\`forge fmt\` output can differ between releases, and this package stores normalised Solidity.
Running with a different forge risks reformatting it.

Either install the pinned version:

  foundryup --install $EXPECTED

or move the pin deliberately (see the header of this script).
EOF
    exit 1
fi

echo "   ✅ forge $ACTUAL matches $PIN_FILE"
