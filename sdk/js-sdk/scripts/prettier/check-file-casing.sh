#!/usr/bin/env bash
#
# check-file-casing.sh
#
# Checks that no TypeScript file name contains two consecutive uppercase letters.
# This enforces the casing rule from naming conventions:
#   e.g. "createEip712.ts" is OK, "createEIP712.ts" is not.
#
# Exit code 0 = all good, 1 = violations found.

set -euo pipefail

# ── Configuration ────────────────────────────────────────────────────────────

# Directories to scan (relative to repo root)
SCAN_DIRS=(
  "src/core"
  "src/ethers"
  "src/viem"
  "test"
)

# File patterns to exclude (glob patterns matched against the full path)
EXCEPTIONS=(
  "test/manual-pack"
)

# ── Helpers ──────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

is_excluded() {
  local file="$1"
  if [[ ${#EXCEPTIONS[@]} -eq 0 ]]; then
    return 1
  fi
  for pattern in "${EXCEPTIONS[@]}"; do
    # shellcheck disable=SC2254
    case "$file" in
      $pattern|$pattern/*) return 0 ;;
    esac
  done
  return 1
}

violations=0

# ── Main ─────────────────────────────────────────────────────────────────────

echo "Checking file name casing..."
echo ""

for dir in "${SCAN_DIRS[@]}"; do
  full_dir="$ROOT_DIR/$dir"
  if [[ ! -d "$full_dir" ]]; then
    echo "Warning: directory not found: $dir"
    continue
  fi

  while IFS= read -r -d '' file; do
    rel_file="${file#"$ROOT_DIR/"}"
    if is_excluded "$rel_file"; then
      continue
    fi

    basename="$(basename "$file")"
    if [[ "$basename" =~ [A-Z]{2} ]]; then
      echo "  $rel_file"
      violations=$((violations + 1))
    fi
  done < <(find "$full_dir" -name '*.ts' -not -name '*.d.ts' -print0)
done

echo ""
if [[ $violations -gt 0 ]]; then
  echo "Found $violations file(s) with consecutive uppercase letters in their name."
  echo "Rule: file names must never contain two consecutive uppercase letters."
  exit 1
else
  echo "All file names follow the casing rule."
  exit 0
fi
