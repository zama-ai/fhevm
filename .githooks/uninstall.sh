#!/bin/sh
#
# Restores the default git hooks directory (.git/hooks/).
#
cd "$(git rev-parse --show-toplevel)" || exit 1
git config --unset core.hooksPath
echo "Git hooks uninstalled: reverted to default .git/hooks/ directory"
