#!/bin/bash

# Define the directory containing the custom hook scripts
HOOKS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Define the Git hooks directory
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

# List of hooks to install
HOOKS=("commit-msg" "pre-push")

# Create the hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install each hook
for hook in "${HOOKS[@]}"; do
  if [ -f "$HOOKS_DIR/$hook" ]; then
    ln -sf "$HOOKS_DIR/$hook" "$GIT_HOOKS_DIR/$hook"
    echo "Installed $hook hook"
  else
    echo "Hook $hook not found in $HOOKS_DIR"
  fi
done

echo "Git hooks installation complete."
