#!/bin/bash
source ./../.env-test

echo $DATABASE_URL

# Version overrides for a fleet that joins a running stack: consensus decides the
# role, and the release has to move too or the cutover is refused. Each is off
# unless its variable is set.
VERSION_OVERRIDE=""
if [[ -n "${BUILD_STACK_VERSION:-}" ]]; then
  VERSION_OVERRIDE="--features fhevm-engine-common/stack-version-override"
fi
if [[ -n "${BUILD_CONSENSUS_VERSION:-}" ]]; then
  VERSION_OVERRIDE="$VERSION_OVERRIDE --features fhevm-engine-common/consensus-version-override"
fi
cargo run --release $VERSION_OVERRIDE -- \
--gateway-url=${GATEWAY_WS_URL} \
--private-key=${TX_SENDER_PRIVATE_KEY} \
--ciphertext-commits-address=${CIPHERTEXT_COMMITS_ADDRESS} \
--input-verification-address=${INPUT_VERIFICATION_ADDRESS} \
--database-url=${DATABASE_URL} \
--database-pool-size=10 \
--database-polling-interval-secs=5 \
--verify-proof-resp-database-channel="event_zkpok_computed" \
--add-ciphertexts-database-channel=event_ciphertexts_uploaded \
--verify-proof-resp-batch-limit=128 \
--verify-proof-resp-max-retries=15 \
--verify-proof-remove-after-max-retries \
--signer-type=private-key
