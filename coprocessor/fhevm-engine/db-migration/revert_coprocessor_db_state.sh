#!/bin/bash
# Reverts coprocessor DB state for a given chain to a given block.
# All data from blocks strictly greater than TO_BLOCK_NUMBER is deleted.
# Pass the block BEFORE the offending one (i.e. offending_block - 1).
#
# Required environment variables:
#   DATABASE_URL    - postgres connection string (e.g. postgres://user:pass@host:5432/coprocessor)
#   CHAIN_ID        - the host chain ID to revert
#   TO_BLOCK_NUMBER - revert to this block (data for blocks > TO_BLOCK_NUMBER is deleted)
#
# Usage (Docker):
#   1. Stop ALL coprocessor services.
#   2. Run:
#      docker run --rm --network <db-network> \
#        -e DATABASE_URL="postgres://user:pass@db-host:5432/coprocessor" \
#        -e CHAIN_ID=12345 \
#        -e TO_BLOCK_NUMBER=500 \
#        ghcr.io/zama-ai/fhevm/coprocessor/db-state-revert:<version>
#   3. Restart coprocessor services.

set -euo pipefail

if [ -z "${DATABASE_URL:-}" ]; then
  echo "ERROR: DATABASE_URL is required"
  exit 1
fi
if [ -z "${CHAIN_ID:-}" ]; then
  echo "ERROR: CHAIN_ID is required"
  exit 1
fi
if [ -z "${TO_BLOCK_NUMBER:-}" ]; then
  echo "ERROR: TO_BLOCK_NUMBER is required"
  exit 1
fi

echo "Reverting chain_id=$CHAIN_ID to block $TO_BLOCK_NUMBER"
psql "$DATABASE_URL" \
  -v chain_id="$CHAIN_ID" \
  -v to_block_number="$TO_BLOCK_NUMBER" \
  -f /revert_coprocessor_db_state.sql
echo "Revert complete"