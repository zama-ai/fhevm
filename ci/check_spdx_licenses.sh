#!/usr/bin/env bash
# Check that all Solidity contracts use the expected SPDX license identifier.

set -euo pipefail

EXPECTED_LICENSE="BSD-3-Clause-Clear"
CONTRACTS_DIR="${1:-contracts}"
EXIT_CODE=0

while IFS= read -r -d '' file; do
    first_line=$(head -n 1 "$file")
    if [[ "$first_line" != "// SPDX-License-Identifier: ${EXPECTED_LICENSE}" ]]; then
        echo "ERROR: Wrong or missing license in $file"
        echo "  Found:    $first_line"
        echo "  Expected: // SPDX-License-Identifier: ${EXPECTED_LICENSE}"
        EXIT_CODE=1
    fi
done < <(find "$CONTRACTS_DIR" -name '*.sol' -print0)

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "All Solidity files use SPDX-License-Identifier: ${EXPECTED_LICENSE}"
fi

exit "$EXIT_CODE"
