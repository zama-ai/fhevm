#!/bin/bash
# Thin wrapper — delegates to the TypeScript e2e runner.
# Usage: ./scripts/e2e-test.sh test input-proof
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
exec bun run "$SCRIPT_DIR/../src/e2e-runner.ts" "$@"
