#!/usr/bin/env bash
#
# check-import-order.sh
#
# Checks that all TypeScript files follow the import ordering rule:
#   1. `import type` statements come first, grouped together
#   2. `import` (value) statements come second, grouped together
#   3. Types and values must NOT be interleaved
#
# Exit code 0 = all good, 1 = violations found.

set -euo pipefail

# ── Configuration ────────────────────────────────────────────────────────────

# Directories to scan (relative to repo root)
SCAN_DIRS=(
  "src/core"
  "src/ethers"
  "src/viem"
  "test/fheTest"
)

# File patterns to exclude (glob patterns matched against the full path)
EXCEPTIONS=(
  "*/index.ts"
  "test/manual-pack"
)

# ── Helpers ──────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

is_excluded() {
  local file="$1"
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
  local state="type" # "type" = still in the type-import section, "value" = switched to value imports
  local lineno=0
  local has_violation=false

  while IFS= read -r line; do
    lineno=$((lineno + 1))

    # Skip blank lines and comments
    [[ -z "$line" || "$line" =~ ^[[:space:]]*//.* ]] && continue

    # Detect import type (including multiline start)
    if [[ "$line" =~ ^import[[:space:]]+type[[:space:]] ]]; then
      if [[ "$state" == "value" ]]; then
        if [[ "$has_violation" == false ]]; then
          echo "  $file"
          has_violation=true
        fi
        echo "    line $lineno: $line"
        violations=$((violations + 1))
      fi
      continue
    fi

    # Detect value import
    if [[ "$line" =~ ^import[[:space:]] ]]; then
      state="value"
      continue
    fi

    # Stop at first non-import line (ignoring blanks/comments handled above)
    # But allow lines that are continuations of multiline imports
    if [[ "$line" =~ ^[[:space:]] ]] || [[ "$line" =~ ^\} ]]; then
      continue
    fi

    break
  done < "$file"
}

# ── Main ─────────────────────────────────────────────────────────────────────

echo "Checking import order..."
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
    check_file "$file"
  done < <(find "$full_dir" -name '*.ts' -not -name '*.d.ts' -print0)
done

echo ""
if [[ $violations -gt 0 ]]; then
  echo "Found $violations import order violation(s)."
  echo "Rule: import type statements must come before value imports, with no interleaving."
  exit 1
else
  echo "All imports are correctly ordered."
  exit 0
fi
