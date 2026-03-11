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

# Get changed files relative to the engine directory
CHANGED_FILES=$(git diff --name-only "$BASE_BRANCH"...HEAD -- "$ENGINE_DIR" 2>/dev/null | \
                sed "s|^coprocessor/fhevm-engine/||" || true)

if [ -z "$CHANGED_FILES" ]; then
    # Fallback: compare working tree if no commits ahead
    CHANGED_FILES=$(git diff --name-only "$BASE_BRANCH" -- "$ENGINE_DIR" 2>/dev/null | \
                    sed "s|^coprocessor/fhevm-engine/||" || true)
fi

if [ -z "$CHANGED_FILES" ]; then
    echo "No changes detected vs $BASE_BRANCH. Nothing to cover."
    exit 0
fi

# Extract unique crate names from changed file paths
# e.g. "host-listener/src/cmd/mod.rs" → "host-listener"
# Root-level files (Cargo.toml, etc.) don't match and are skipped
CHANGED_CRATES=$(echo "$CHANGED_FILES" | sed -n 's|^\([^/]*\)/.*|\1|p' | sort -u)

# Check for root-level changes that affect all crates (e.g. Cargo.lock)
# Cargo.toml profile/toolchain changes don't require full workspace coverage
ROOT_CRITICAL=$(echo "$CHANGED_FILES" | grep -v '/' | grep -v '\.toml$' || true)
if echo "$CHANGED_FILES" | grep -q '^Cargo\.lock$'; then
    ROOT_CRITICAL="Cargo.lock"
fi
if [ -n "$ROOT_CRITICAL" ]; then
    echo "Workspace config changed ($ROOT_CRITICAL) — running full workspace coverage."
    echo ""
    exec make coverage
fi

if [ -z "$CHANGED_CRATES" ]; then
    echo "No crate-level changes detected. Nothing to cover."
    exit 0
fi

echo "Changed crates:"
echo "$CHANGED_CRATES" | sed 's/^/  - /'
echo ""

# If fhevm-engine-common changed, run full workspace (all crates depend on it)
if echo "$CHANGED_CRATES" | grep -q "^fhevm-engine-common$"; then
    echo "fhevm-engine-common changed — running full workspace coverage."
    echo ""
    exec make coverage
fi

# Build --package flags for each changed crate
PKG_FLAGS=""
for crate in $CHANGED_CRATES; do
    # Skip non-crate directories (e.g. db-migration, scripts)
    if [ -f "$crate/Cargo.toml" ]; then
        PKG_FLAGS="$PKG_FLAGS --package $crate"
    else
        echo "  Skipping $crate (not a Cargo crate)"
    fi
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

echo ""
echo "Coverage report saved to coverage-report.txt"
tail -1 coverage-report.txt
