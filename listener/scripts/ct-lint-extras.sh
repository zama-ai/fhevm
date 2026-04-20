#!/usr/bin/env bash
# Extra lint checks invoked by chart-testing (ct lint).
# ct passes the chart path as $1.
set -euo pipefail

CHART_DIR="${1:?usage: ct-lint-extras.sh <chart-path>}"
TARGET_BRANCH="${CT_TARGET_BRANCH:-main}"

# ── 1. Broken symlink check ─────────────────────────────────────────────────
echo "── symlink check: $CHART_DIR ──"
broken=0
while IFS= read -r link; do
    [ -z "$link" ] && continue
    if [ ! -e "$link" ]; then
        echo "ERROR: broken symlink: $link -> $(readlink "$link")"
        broken=1
    fi
done < <(find "$CHART_DIR" -type l 2>/dev/null)

if [ $broken -ne 0 ]; then
    exit 1
fi
echo "OK: all symlinks resolve"

# ── 2. Symlink target version-bump check ─────────────────────────────────────
# ct only detects changes inside chart-dirs. If a symlink target outside the
# chart changed, ct won't flag the chart as modified and check-version-increment
# never fires. This section covers that gap.
echo ""
echo "── symlink target change check: $CHART_DIR ──"

merge_base=$(git merge-base HEAD "origin/${TARGET_BRANCH}" 2>/dev/null || echo "")
if [ -z "$merge_base" ]; then
    echo "SKIP: could not find merge base with origin/${TARGET_BRANCH}"
    exit 0
fi

targets_changed=0
while IFS= read -r link; do
    [ -z "$link" ] && continue
    target=$(readlink "$link")
    # Resolve to repo-relative path
    abs_target=$(cd "$(dirname "$link")" && realpath -q "$target" 2>/dev/null || echo "")
    [ -z "$abs_target" ] && continue
    repo_root=$(git rev-parse --show-toplevel)
    rel_target="${abs_target#"${repo_root}"/}"
    # Check if this target changed vs the target branch
    if git diff --quiet "$merge_base" -- "$rel_target" 2>/dev/null; then
        :
    else
        echo "CHANGED: symlink target $rel_target (via $link)"
        targets_changed=1
    fi
done < <(find "$CHART_DIR" -type l 2>/dev/null)

if [ $targets_changed -eq 1 ]; then
    old_ver=$(git show "${merge_base}:${CHART_DIR}/Chart.yaml" 2>/dev/null | grep '^version:' | awk '{print $2}')
    new_ver=$(grep '^version:' "${CHART_DIR}/Chart.yaml" | awk '{print $2}')
    if [ "$old_ver" = "$new_ver" ]; then
        echo "FAIL: symlink targets outside chart changed but Chart.yaml version is still $old_ver"
        exit 1
    fi
    echo "OK: version bumped $old_ver -> $new_ver"
else
    echo "OK: no symlink targets changed"
fi
