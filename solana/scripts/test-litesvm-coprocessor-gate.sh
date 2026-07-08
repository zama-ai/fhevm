#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if ! docker info >/dev/null 2>&1; then
  echo "Docker must be running; the tfhe-worker test harness starts Postgres with testcontainers." >&2
  exit 1
fi

cd "$ROOT/solana"
NO_DNA=1 bash scripts/check-zama-host-idl.sh

cd "$ROOT/coprocessor/fhevm-engine"
tests=(
  solana_fhe_eval_replays_threshold_logs_from_litesvm_metadata
  solana_fhe_rand_creates_ciphertext_and_decrypts
  solana_user_decrypt_acl_invariants_match_evm_semantics
)

for test in "${tests[@]}"; do
  NO_DNA=1 SQLX_OFFLINE=true cargo test -p tfhe-worker "$test" -- --ignored --test-threads=1 --nocapture
done
