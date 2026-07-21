#!/usr/bin/env bash
# Offline tests for select-overrides.sh: injects CHANGED_FILES and a stubbed image-existence
# probe (MANIFEST_INSPECT), then asserts the computed override set and lock pins.
#
# Run: bash solana/scripts/e2e/select-overrides.test.sh
set -euo pipefail

SCRIPT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/select-overrides.sh"
BASE_SHA="0123456789abcdef0123456789abcdef01234567"
failures=0
tests_run=0

# Stub manifest probes. "true" pretends every image exists; the stubs below simulate publish
# gaps to exercise the build-everything fallback.
STUB_DIR="$(mktemp -d)"
trap 'rm -rf "$STUB_DIR"' EXIT
cat > "$STUB_DIR/missing-all" <<'EOF'
#!/usr/bin/env bash
exit 1
EOF
# One group's base-tag image missing (relayer) while every other image exists: a partial publish
# must never mix a stale group in — the whole run falls back to source builds.
cat > "$STUB_DIR/missing-relayer" <<'EOF'
#!/usr/bin/env bash
case "$1" in ghcr.io/zama-ai/fhevm/relayer:*) exit 1 ;; *) exit 0 ;; esac
EOF
chmod +x "$STUB_DIR/missing-all" "$STUB_DIR/missing-relayer"

check() {
  local name="$1" changed="$2" probe="$3" want_overrides="$4" want_pins="$5"
  local out overrides pins
  tests_run=$((tests_run + 1))
  out="$(CHANGED_FILES="$changed" MANIFEST_INSPECT="$probe" GITHUB_OUTPUT="" bash "$SCRIPT" "$BASE_SHA")"
  overrides="$(printf '%s\n' "$out" | sed -n 's/^overrides=//p')"
  pins="$(printf '%s\n' "$out" | sed -n 's/^lock_pins=//p')"
  if [ "$overrides" = "$want_overrides" ] && [ "$pins" = "$want_pins" ]; then
    echo "ok   $name"
  else
    echo "FAIL $name"
    echo "  want overrides: $want_overrides"
    echo "  got  overrides: $overrides"
    echo "  want lock_pins: $want_pins"
    echo "  got  lock_pins: $pins"
    failures=$((failures + 1))
  fi
}

ALL="gateway-contracts host-contracts coprocessor relayer kms-connector"
T="feature-solana-$BASE_SHA"
PINS_GATEWAY="GATEWAY_VERSION=$T"
PINS_HOST="HOST_VERSION=$T"
PINS_COPRO="COPROCESSOR_DB_MIGRATION_VERSION=$T COPROCESSOR_HOST_LISTENER_VERSION=$T COPROCESSOR_GW_LISTENER_VERSION=$T COPROCESSOR_TFHE_WORKER_VERSION=$T COPROCESSOR_ZKPROOF_WORKER_VERSION=$T COPROCESSOR_SNS_WORKER_VERSION=$T COPROCESSOR_TX_SENDER_VERSION=$T"
PINS_RELAYER="RELAYER_VERSION=$T RELAYER_MIGRATE_VERSION=$T"
PINS_CONNECTOR="CONNECTOR_DB_MIGRATION_VERSION=$T CONNECTOR_GW_LISTENER_VERSION=$T CONNECTOR_KMS_WORKER_VERSION=$T CONNECTOR_TX_SENDER_VERSION=$T"

# Solana-only script/geyser/docs change: nothing is built, everything pinned to the base tag.
check "solana scripts only -> no overrides" \
  $'solana/scripts/e2e/full-vertical.sh\nsolana/geyser/src/lib.rs\nsolana/docs/notes.md' \
  "true" \
  "none" \
  "$PINS_GATEWAY $PINS_HOST $PINS_COPRO $PINS_RELAYER $PINS_CONNECTOR"

# zama-host is compiled into the coprocessor and relayer binaries, but NOT the kms-connector's
# (its image copies only solana/crates/zama-solana-acl).
check "solana program change -> coprocessor + relayer" \
  "solana/programs/zama-host/src/lib.rs" \
  "true" \
  "coprocessor relayer" \
  "$PINS_GATEWAY $PINS_HOST $PINS_CONNECTOR"

# zama-solana-acl is the one solana crate the kms-connector image consumes.
check "zama-solana-acl change -> all rust consumers" \
  "solana/crates/zama-solana-acl/src/lib.rs" \
  "true" \
  "coprocessor kms-connector relayer" \
  "$PINS_GATEWAY $PINS_HOST"

check "other solana crate change -> coprocessor + relayer" \
  "solana/crates/zama-fhe/src/lib.rs" \
  "true" \
  "coprocessor relayer" \
  "$PINS_GATEWAY $PINS_HOST $PINS_CONNECTOR"

check "solana Cargo.lock change -> coprocessor + relayer" \
  "solana/Cargo.lock" \
  "true" \
  "coprocessor relayer" \
  "$PINS_GATEWAY $PINS_HOST $PINS_CONNECTOR"

# On-chain-only demo programs are compiled into no docker image.
check "demo-vault change -> no overrides" \
  "solana/programs/demo-vault/src/lib.rs" \
  "true" \
  "none" \
  "$PINS_GATEWAY $PINS_HOST $PINS_COPRO $PINS_RELAYER $PINS_CONNECTOR"

check "confidential-deposit-app change -> no overrides" \
  "solana/programs/confidential-deposit-app/src/lib.rs" \
  "true" \
  "none" \
  "$PINS_GATEWAY $PINS_HOST $PINS_COPRO $PINS_RELAYER $PINS_CONNECTOR"

check "coprocessor change -> coprocessor only" \
  "coprocessor/fhevm-engine/tfhe-worker/src/main.rs" \
  "true" \
  "coprocessor" \
  "$PINS_GATEWAY $PINS_HOST $PINS_RELAYER $PINS_CONNECTOR"

check "kms-connector change -> kms-connector only" \
  "kms-connector/crates/gw-listener/src/lib.rs" \
  "true" \
  "kms-connector" \
  "$PINS_GATEWAY $PINS_HOST $PINS_COPRO $PINS_RELAYER"

# Contracts are compiled into the Rust images (bindings/artifacts), so they fan out.
check "host-contracts change -> contracts + rust consumers" \
  "host-contracts/contracts/FHEVMExecutor.sol" \
  "true" \
  "host-contracts coprocessor kms-connector relayer" \
  "$PINS_GATEWAY"

check "gateway-contracts change -> contracts + rust consumers" \
  "gateway-contracts/contracts/Decryption.sol" \
  "true" \
  "gateway-contracts coprocessor kms-connector relayer" \
  "$PINS_HOST"

check "sdk-only change -> no overrides" \
  "sdk/js-sdk/src/solana/index.ts" \
  "true" \
  "none" \
  "$PINS_GATEWAY $PINS_HOST $PINS_COPRO $PINS_RELAYER $PINS_CONNECTOR"

# Build-recipe changes force full source builds regardless of published images.
check "workflow change -> build all" \
  ".github/workflows/solana-e2e.yml" \
  "true" \
  "$ALL" \
  ""

check "test-suite/fhevm change -> build all" \
  "test-suite/fhevm/src/generate/compose.ts" \
  "true" \
  "$ALL" \
  ""

check "clean-e2e.sh change -> build all" \
  "solana/scripts/e2e/clean-e2e.sh" \
  "true" \
  "$ALL" \
  ""

# Fail-safe: an incomplete base-commit image set means build everything — the floating tag is
# never consumed (a partial publish could leave it mixing branch commits).
check "no published images -> build all" \
  "solana/docs/notes.md" \
  "$STUB_DIR/missing-all" \
  "$ALL" \
  ""

check "one group's image missing -> build all (no tag mixing)" \
  "solana/docs/notes.md" \
  "$STUB_DIR/missing-relayer" \
  "$ALL" \
  ""

if [ "$failures" -gt 0 ]; then
  echo "$failures of $tests_run select-overrides test(s) failed"
  exit 1
fi
echo "all $tests_run select-overrides tests passed"
