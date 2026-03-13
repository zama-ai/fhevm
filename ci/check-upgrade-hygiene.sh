#!/usr/bin/env bash
# Checks that upgradeable contracts have proper version bumps when bytecode changes.
# Usage: ./ci/check-upgrade-hygiene.sh <main-pkg-dir> <pr-pkg-dir>
set -euo pipefail

MAIN_DIR="$1"
PR_DIR="$2"

if [ ! -f "$PR_DIR/upgrade-manifest.json" ]; then
  echo "::error::upgrade-manifest.json not found in $PR_DIR"
  exit 1
fi

ERRORS=0

# Extract REINITIALIZER_VERSION, MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION in one pass.
extract_versions() {
  awk '
    function val_after_eq(line) { sub(/.*=[[:space:]]*/, "", line); sub(/[^0-9].*/, "", line); return line }
    /REINITIALIZER_VERSION[[:space:]]*=[[:space:]]*[0-9]/ { reinit = val_after_eq($0) }
    /MAJOR_VERSION[[:space:]]*=[[:space:]]*[0-9]/         { major  = val_after_eq($0) }
    /MINOR_VERSION[[:space:]]*=[[:space:]]*[0-9]/         { minor  = val_after_eq($0) }
    /PATCH_VERSION[[:space:]]*=[[:space:]]*[0-9]/         { patch  = val_after_eq($0) }
    END { print reinit, major, minor, patch }
  ' "$1"
}

for name in $(jq -r '.[]' "$PR_DIR/upgrade-manifest.json"); do
  echo "::group::Checking $name"

  main_sol="$MAIN_DIR/contracts/${name}.sol"
  pr_sol="$PR_DIR/contracts/${name}.sol"

  if [ ! -f "$main_sol" ]; then
    echo "Skipping $name (new contract, not on main)"
    echo "::endgroup::"
    continue
  fi

  if [ ! -f "$pr_sol" ]; then
    echo "::error::$name listed in upgrade-manifest.json but missing in PR"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi

  read -r main_reinit main_major main_minor main_patch < <(extract_versions "$main_sol")
  read -r pr_reinit pr_major pr_minor pr_patch < <(extract_versions "$pr_sol")

  for var in main_reinit pr_reinit main_major pr_major main_minor pr_minor main_patch pr_patch; do
    if [ -z "${!var}" ]; then
      echo "::error::Failed to parse $var for $name"
      ERRORS=$((ERRORS + 1))
      echo "::endgroup::"
      continue 2
    fi
  done

  # forge inspect compiles on first call and caches; stderr suppressed to avoid warning noise.
  if ! main_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$MAIN_DIR" deployedBytecode 2>/dev/null); then
    echo "::error::Failed to compile $name on main"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi
  if ! pr_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$PR_DIR" deployedBytecode 2>/dev/null); then
    echo "::error::Failed to compile $name on PR"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi

  if [ "$main_bytecode" = "$pr_bytecode" ]; then
    echo "$name: bytecode unchanged"
    if [ "$main_reinit" != "$pr_reinit" ]; then
      echo "::error::$name REINITIALIZER_VERSION bumped ($main_reinit -> $pr_reinit) but bytecode is unchanged"
      ERRORS=$((ERRORS + 1))
    fi
    echo "::endgroup::"
    continue
  fi

  echo "$name: bytecode CHANGED"

  if [ "$main_reinit" = "$pr_reinit" ]; then
    echo "::error::$name bytecode changed but REINITIALIZER_VERSION was not bumped (still $pr_reinit)"
    ERRORS=$((ERRORS + 1))
  else
    # Convention: reinitializeV{N-1} for REINITIALIZER_VERSION=N
    expected_fn="reinitializeV$((pr_reinit - 1))"
    if ! grep -qE "function[[:space:]]+${expected_fn}[[:space:]]*\(" "$pr_sol"; then
      echo "::error::$name has REINITIALIZER_VERSION=$pr_reinit but no $expected_fn() function found"
      ERRORS=$((ERRORS + 1))
    fi
  fi

  if [ "$main_major" = "$pr_major" ] && [ "$main_minor" = "$pr_minor" ] && [ "$main_patch" = "$pr_patch" ]; then
    echo "::error::$name bytecode changed but semantic version was not bumped (still v${pr_major}.${pr_minor}.${pr_patch})"
    ERRORS=$((ERRORS + 1))
  fi

  echo "::endgroup::"
done

if [ "$ERRORS" -gt 0 ]; then
  echo "::error::Upgrade hygiene check failed with $ERRORS error(s)"
  exit 1
fi

echo "All contracts passed upgrade hygiene checks"
