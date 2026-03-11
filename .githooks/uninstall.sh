#!/bin/sh
#
# Uninstall coverage hooks and revert to the default .git/hooks/ directory.
#

git config --unset core.hooksPath
echo "Reverted git hooks to default (.git/hooks/)."
