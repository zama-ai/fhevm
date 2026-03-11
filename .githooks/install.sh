#!/bin/sh
#
# Install coverage pre-push hook by setting core.hooksPath.
#
# WARNING: This replaces any existing git hooks (including those
# installed via .github/hooks/install.sh). To revert, run uninstall.sh.
#

echo "WARNING: Setting core.hooksPath to .githooks/ will replace any existing git hooks."
echo "  To revert, run: sh .githooks/uninstall.sh"
echo ""

git config core.hooksPath .githooks
echo "Git hooks path set to .githooks/"
chmod +x .githooks/pre-push
echo "Installed pre-push hook (coverage check)."
