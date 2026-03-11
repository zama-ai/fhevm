#!/bin/bash
#
# Install git hooks from .github/hooks/ into .git/hooks/ via symlinks.
#
# Usage:
#   sh .github/hooks/install.sh                                  # Install all hooks
#   sh .github/hooks/install.sh pre-push-coverage                # Install only coverage hook
#   sh .github/hooks/install.sh pre-push                         # Install only quality gates hook
#   sh .github/hooks/install.sh pre-push pre-push-coverage       # Install both pre-push hooks
#
# Available hooks:
#   commit-msg           — Angular conventional commit format check
#   pre-push             — Quality gates (cargo fmt, clippy, test) [blocking]
#   pre-push-coverage    — Coverage report check [non-blocking]
#

HOOKS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_HOOKS_DIR="$(git rev-parse --git-dir)/hooks"

mkdir -p "$GIT_HOOKS_DIR"

install_symlink() {
  local source="$1"
  local target="$2"

  if [ ! -f "$HOOKS_DIR/$source" ]; then
    echo "Hook '$source' not found in $HOOKS_DIR"
    return 1
  fi

  ln -sf "$HOOKS_DIR/$source" "$GIT_HOOKS_DIR/$target"
  echo "Installed $source hook"
}

# Collect requested hooks
if [ -n "$1" ]; then
  REQUESTED="$*"
else
  REQUESTED="commit-msg pre-push"
fi

# Check if both pre-push hooks are requested
HAS_PREPUSH=false
HAS_COVERAGE=false
for hook in $REQUESTED; do
  case "$hook" in
    pre-push) HAS_PREPUSH=true ;;
    pre-push-coverage) HAS_COVERAGE=true ;;
  esac
done

# Install non-pre-push hooks as symlinks
for hook in $REQUESTED; do
  case "$hook" in
    pre-push|pre-push-coverage) ;; # handled below
    *) install_symlink "$hook" "$hook" ;;
  esac
done

# Install pre-push hook(s)
if $HAS_PREPUSH && $HAS_COVERAGE; then
  # Both requested — create a wrapper that runs both
  cat > "$GIT_HOOKS_DIR/pre-push" << 'WRAPPER'
#!/bin/sh
# Auto-generated wrapper — runs both pre-push hooks.
# Re-run install.sh to regenerate.

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_SRC="$REPO_ROOT/.github/hooks"

# Coverage check (non-blocking)
"$HOOKS_SRC/pre-push-coverage" "$@"

# Quality gates (blocking)
"$HOOKS_SRC/pre-push" "$@" || exit $?
exit 0
WRAPPER
  chmod +x "$GIT_HOOKS_DIR/pre-push"
  echo "Installed pre-push + pre-push-coverage hooks (combined)"
elif $HAS_PREPUSH; then
  install_symlink "pre-push" "pre-push"
elif $HAS_COVERAGE; then
  install_symlink "pre-push-coverage" "pre-push"
fi

echo "Git hooks installation complete."
