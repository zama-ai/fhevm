#!/usr/bin/env bash
# Check that all Solidity contracts use the expected SPDX license identifier.

set -euo pipefail

EXPECTED_LICENSE="BSD-3-Clause-Clear"
EXIT_CODE=0
DIRS=()
EXCLUDES=()

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --exclude)
            EXCLUDES+=("$2")
            shift 2
            ;;
        *)
            DIRS+=("$1")
            shift
            ;;
    esac
done

# Default to contracts/ if no directories specified
if [[ ${#DIRS[@]} -eq 0 ]]; then
    DIRS=("contracts")
fi

for dir in "${DIRS[@]}"; do
    while IFS= read -r -d '' file; do
        # Check if file matches any exclude pattern
        skip=false
        for exclude in ${EXCLUDES[@]+"${EXCLUDES[@]}"}; do
            if [[ "$file" == *"$exclude"* ]]; then
                skip=true
                break
            fi
        done
        if "$skip"; then
            continue
        fi

        first_line=$(head -n 1 "$file")
        if [[ "$first_line" != "// SPDX-License-Identifier: ${EXPECTED_LICENSE}" ]]; then
            echo "ERROR: Wrong or missing license in $file"
            echo "  Found:    $first_line"
            echo "  Expected: // SPDX-License-Identifier: ${EXPECTED_LICENSE}"
            EXIT_CODE=1
        fi
    done < <(find "$dir" -name '*.sol' -print0 | sort -z)
done

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "All Solidity files use SPDX-License-Identifier: ${EXPECTED_LICENSE}"
fi

exit "$EXIT_CODE"
