#!/bin/sh
#
# Sets up .githooks/ as the git hooks directory for this repository.
#
cd "$(git rev-parse --show-toplevel)" || exit 1
git config core.hooksPath .githooks
chmod +x .githooks/pre-push
echo "Git hooks installed: using .githooks/ directory"
