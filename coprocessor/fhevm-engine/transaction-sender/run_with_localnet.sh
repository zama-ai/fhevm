#!/bin/bash
source ./../.env-test

echo $DATABASE_URL

# stack-version-override lets BUILD_STACK_VERSION from the environment override
# the hard-coded stack version baked into the binary (see fhevm-engine-common).
cargo run --release --features fhevm-engine-common/stack-version-override -- \
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
