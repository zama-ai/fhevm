#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

export NO_DNA="${NO_DNA:-1}"
export SQLX_OFFLINE="${SQLX_OFFLINE:-true}"
if command -v mold >/dev/null 2>&1; then
  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER:-clang}"
  export RUSTFLAGS="${RUSTFLAGS:--C link-arg=-fuse-ld=mold}"
fi

if ! docker info >/dev/null 2>&1; then
  echo "Docker must be running; the tfhe-worker test harness starts Postgres with testcontainers." >&2
  exit 1
fi

cd "$ROOT/solana"
bash scripts/check-zama-host-idl.sh

cd "$ROOT/coprocessor/fhevm-engine"
tests=(
  solana_confidential_transfer_with_real_ciphertexts_computes_and_decrypts
  solana_fhe_eval_replays_threshold_logs_from_litesvm_metadata
  solana_fhe_rand_creates_ciphertext_and_decrypts
  solana_trivial_encrypt_then_confidential_transfer_computes_and_decrypts
  solana_user_decrypt_acl_invariants_match_evm_semantics
)

for test in "${tests[@]}"; do
  cargo test --profile local -p tfhe-worker "$test" -- --ignored --test-threads=1 --nocapture
done
