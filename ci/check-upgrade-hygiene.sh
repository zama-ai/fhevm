#!/usr/bin/env bash
# ci/check-upgrade-hygiene.sh
#
# Validates that upgradeable contracts have proper version bumps when bytecode changes.
# Compares deployed bytecodes between two copies of a contract package (e.g. main vs PR branch).
#
# Usage:
#   ./ci/check-upgrade-hygiene.sh <main-pkg-dir> <pr-pkg-dir>
#
# Example:
#   ./ci/check-upgrade-hygiene.sh main-branch/host-contracts host-contracts
#
# Requires: forge (Foundry), jq
# Both directories must have:
#   - foundry.toml with cbor_metadata=false and bytecode_hash='none'
#   - upgrade-manifest.json listing contract names
#   - contracts/<Name>.sol for each manifest entry
#   - addresses/ stub (generated file) so contracts compile

set -euo pipefail

MAIN_DIR="$1"
PR_DIR="$2"

if [ ! -f "$PR_DIR/upgrade-manifest.json" ]; then
  echo "::error::upgrade-manifest.json not found in $PR_DIR"
  exit 1
fi

ERRORS=0

# Extract all four version constants from a .sol file in a single pass.
# Returns: REINITIALIZER_VERSION MAJOR_VERSION MINOR_VERSION PATCH_VERSION
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

# Pre-compile both roots in parallel so all forge inspect calls are cache hits.
forge build --root "$MAIN_DIR" &
pid_main=$!
forge build --root "$PR_DIR" &
pid_pr=$!
wait "$pid_main" || { echo "::error::forge build failed for $MAIN_DIR"; exit 1; }
wait "$pid_pr"   || { echo "::error::forge build failed for $PR_DIR"; exit 1; }

for name in $(jq -r '.[]' "$PR_DIR/upgrade-manifest.json"); do
  echo "::group::Checking $name"

  main_sol="$MAIN_DIR/contracts/${name}.sol"
  pr_sol="$PR_DIR/contracts/${name}.sol"

  # Skip contracts not present on main (newly added)
  if [ ! -f "$main_sol" ]; then
    echo "Skipping $name (new contract, not on main)"
    echo "::endgroup::"
    continue
  fi

  if [ ! -f "$pr_sol" ]; then
    echo "::error::$name is in upgrade-manifest.json but contracts/${name}.sol not found in PR"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi

  # --- Extract version constants from both (single pass per file) ---
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

  # --- Compare bytecodes (paths relative to --root, builds are cached) ---
  # Capture only stdout for bytecode; stderr goes to /dev/null on success (warnings),
  # but on failure we re-run to capture the error message.
  if ! main_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$MAIN_DIR" deployedBytecode 2>/dev/null); then
    echo "::error::Failed to inspect $name on main:$(forge inspect "contracts/${name}.sol:$name" --root "$MAIN_DIR" deployedBytecode 2>&1 || true)"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi
  if ! pr_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$PR_DIR" deployedBytecode 2>/dev/null); then
    echo "::error::Failed to inspect $name on PR:$(forge inspect "contracts/${name}.sol:$name" --root "$PR_DIR" deployedBytecode 2>&1 || true)"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi

  bytecode_changed=false
  if [ "$main_bytecode" != "$pr_bytecode" ]; then
    bytecode_changed=true
  fi

  version_changed=false
  if [ "$main_major" != "$pr_major" ] || [ "$main_minor" != "$pr_minor" ] || [ "$main_patch" != "$pr_patch" ]; then
    version_changed=true
  fi

  reinit_changed=false
  if [ "$main_reinit" != "$pr_reinit" ]; then
    reinit_changed=true
  fi

  if [ "$bytecode_changed" = true ]; then
    echo "$name: bytecode CHANGED"

    # Check 1: REINITIALIZER_VERSION must be bumped
    if [ "$reinit_changed" = false ]; then
      echo "::error::$name bytecode changed but REINITIALIZER_VERSION was not bumped (still $pr_reinit)"
      ERRORS=$((ERRORS + 1))
    fi

    # Check 2: reinitializeVN function must exist (convention: N = REINITIALIZER_VERSION - 1)
    if [ "$reinit_changed" = true ]; then
      expected_n=$((pr_reinit - 1))
      expected_fn="reinitializeV${expected_n}"
      # Look for function declaration (not just any mention)
      if ! grep -qE "function[[:space:]]+${expected_fn}[[:space:]]*\(" "$pr_sol"; then
        echo "::error::$name has REINITIALIZER_VERSION=$pr_reinit but no $expected_fn() function found"
        ERRORS=$((ERRORS + 1))
      fi
    fi

    # Check 3: Semantic version must be bumped
    if [ "$version_changed" = false ]; then
      echo "::error::$name bytecode changed but semantic version was not bumped (still v${pr_major}.${pr_minor}.${pr_patch})"
      ERRORS=$((ERRORS + 1))
    fi

  else
    echo "$name: bytecode unchanged"

    # Inverse check: reinitializer should NOT be bumped if bytecode didn't change
    if [ "$reinit_changed" = true ]; then
      echo "::error::$name REINITIALIZER_VERSION bumped ($main_reinit -> $pr_reinit) but bytecode is unchanged"
      ERRORS=$((ERRORS + 1))
    fi
  fi

  echo "::endgroup::"
done

if [ "$ERRORS" -gt 0 ]; then
  echo "::error::Upgrade hygiene check failed with $ERRORS error(s)"
  exit 1
fi

echo "All contracts passed upgrade hygiene checks"
