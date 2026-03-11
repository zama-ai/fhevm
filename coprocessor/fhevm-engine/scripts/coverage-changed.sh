#!/bin/sh
#
# Detect which workspace crates have changed vs main and run coverage
# only for those crates. If fhevm-engine-common changes, run all.
#
set -e

BASE_BRANCH="${BASE_BRANCH:-main}"
REPO_ROOT="$(git rev-parse --show-toplevel)"
ENGINE_DIR="$REPO_ROOT/coprocessor/fhevm-engine"

cd "$ENGINE_DIR"

# Fetch latest remote to ensure accurate comparison
git fetch origin "$BASE_BRANCH" --quiet 2>/dev/null || true

# Compare only your branch's changes against the remote base branch
# Uses three-dot diff against origin/ to exclude changes already on main
CHANGED_FILES=$(git diff --name-only "origin/$BASE_BRANCH"...HEAD -- "$ENGINE_DIR" 2>/dev/null | \
                sed "s|^coprocessor/fhevm-engine/||" || true)

if [ -z "$CHANGED_FILES" ]; then
    echo "No changes detected vs $BASE_BRANCH. Nothing to cover."
    exit 0
fi

# Extract unique directory names from changed file paths, filtered to actual crates
# e.g. "host-listener/src/cmd/mod.rs" → "host-listener" (only if host-listener/Cargo.toml exists)
# Root-level files (Cargo.toml, Makefile, etc.) are skipped by the sed filter
ALL_DIRS=$(echo "$CHANGED_FILES" | sed -n 's|^\([^/]*\)/.*|\1|p' | sort -u)

# Only Cargo.lock affects all crates — other root-level files are ignored
if echo "$CHANGED_FILES" | grep -q '^Cargo\.lock$'; then
    echo "Cargo.lock changed — running full workspace coverage."
    echo ""
    exec make coverage
fi

# Filter to actual crates (directories with Cargo.toml)
CHANGED_CRATES=""
for dir in $ALL_DIRS; do
    if [ -f "$dir/Cargo.toml" ]; then
        CHANGED_CRATES="$CHANGED_CRATES $dir"
    fi
done
CHANGED_CRATES=$(echo "$CHANGED_CRATES" | xargs)

if [ -z "$CHANGED_CRATES" ]; then
    echo "No crate-level changes detected. Nothing to cover."
    exit 0
fi

echo "Changed crates:"
echo "$CHANGED_CRATES" | tr ' ' '\n' | sed 's/^/  - /'
echo ""

# If fhevm-engine-common changed, run full workspace (all crates depend on it)
if echo "$CHANGED_CRATES" | grep -q "fhevm-engine-common"; then
    echo "fhevm-engine-common changed — running full workspace coverage."
    echo ""
    exec make coverage
fi

# Build --package flags
PKG_FLAGS=""
for crate in $CHANGED_CRATES; do
    PKG_FLAGS="$PKG_FLAGS --package $crate"
done

if [ -z "$PKG_FLAGS" ]; then
    echo "No Cargo crates changed. Nothing to cover."
    exit 0
fi

echo "Running coverage for:$PKG_FLAGS"
echo ""

cargo llvm-cov clean --workspace --profile coverage

DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor \
TEST_GLOBAL_LOCALSTACK=1 \
cargo llvm-cov --no-report $PKG_FLAGS --profile coverage -- --test-threads=1

cargo llvm-cov report --profile coverage 2>&1 | tee coverage-report.txt
echo "# commit: $(git rev-parse HEAD)" >> coverage-report.txt

echo ""
echo "Coverage report saved to coverage-report.txt"
tail -2 coverage-report.txt
