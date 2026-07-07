#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if ! docker info >/dev/null 2>&1; then
  echo "Docker must be running; the tfhe-worker test harness starts Postgres with testcontainers." >&2
  exit 1
fi

cd "$ROOT/solana"
bash scripts/check-zama-host-idl.sh

cd "$ROOT/coprocessor/fhevm-engine"
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_poc -- --ignored --test-threads=1
