#!/bin/bash
#
# diff-test.sh — Structural diff between the bash deploy script and the
# TypeScript CLI's --dry-run output.
#
# Usage:  ./scripts/diff-test.sh
#
# Runs `bun test src/__tests__/diff.test.ts` which does the actual comparison.
#
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
FHEVM_DIR="$SCRIPT_DIR/.."

cd "$FHEVM_DIR"
exec bun test src/__tests__/diff.test.ts
