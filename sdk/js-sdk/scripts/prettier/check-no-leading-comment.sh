#!/usr/bin/env bash
#
# check-no-leading-comment.sh
#
# Checks that no TypeScript file starts with a comment.
# The first non-empty line must not be a single-line (//) or multi-line (/*) comment.
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
  "src/core/host-contracts/abi-fragments/fragments.ts"
  "src/core/_version.ts"
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
      $pattern) return 0 ;;
    esac
  done
  return 1
}

violations=0

check_file() {
  local file="$1"
  local rel_file="$2"

  while IFS= read -r line; do
    # Skip empty / whitespace-only lines
    [[ "$line" =~ ^[[:space:]]*$ ]] && continue

    # First non-empty line: check for comment (eslint-disable and jsdoc are allowed)
    if [[ "$line" =~ ^[[:space:]]*/[/\*] ]] && ! [[ "$line" =~ eslint-disable ]] && ! [[ "$line" =~ ^[[:space:]]*/\*\* ]]; then
      echo "  $rel_file"
      violations=$((violations + 1))
    fi
    return
  done < "$file"
}

# ── Main ─────────────────────────────────────────────────────────────────────

echo "Checking for files starting with a comment..."
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
    check_file "$file" "$rel_file"
  done < <(find "$full_dir" -name '*.ts' -not -name '*.d.ts' -print0)
done

echo ""
if [[ $violations -gt 0 ]]; then
  echo "Found $violations file(s) starting with a comment."
  echo "Rule: files must not start with a comment."
  exit 1
else
  echo "No files start with a comment."
  exit 0
fi
