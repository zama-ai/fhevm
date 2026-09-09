#!/bin/bash
set -euo pipefail
source ./../.env-test

# The detector resolves each operator's S3 bucket URL from the on-chain
# GatewayConfig, which the e2e sets to `http://minio:9000/coproc-<N>`. That hostname
# only resolves inside the Docker network — on the host `minio` is unresolvable,
# so every state_hash GET fails with "error sending request for url
# (http://minio:9000/...)". minio is published on the host at localhost:9000, so
# we alias `minio` -> localhost via HOSTALIASES (honored by getaddrinfo, which
# reqwest uses) instead of touching /etc/hosts. Single-label names only — `minio`
# qualifies.
HOSTALIASES_FILE="$(mktemp)"
printf 'minio localhost\n' > "${HOSTALIASES_FILE}"
export HOSTALIASES="${HOSTALIASES_FILE}"
trap 'rm -f "${HOSTALIASES_FILE}"' EXIT

echo "DATABASE_URL=$DATABASE_URL"
echo "GATEWAY_WS_URL=$GATEWAY_WS_URL"
echo "GATEWAY_CONFIG_ADDRESS=$GATEWAY_CONFIG_ADDRESS"
echo "HOSTALIASES=$HOSTALIASES ($(cat "${HOSTALIASES_FILE}"))"

# --my-bucket / --s3-endpoint mirror the docker-compose consensus-detector so the
# state_hash worker can upload to minio from the host (path-style, host endpoint).
# AWS_* creds + region come from ../.env-test.
# stack-version-override lets BUILD_STACK_VERSION from the environment override
# the hard-coded stack version baked into the binary (see fhevm-engine-common).
cargo run --release --features fhevm-engine-common/stack-version-override -- \
--database-url=${DATABASE_URL} \
--database-pool-size=4 \
--gw-url=${GATEWAY_WS_URL} \
--gateway-config-address=${GATEWAY_CONFIG_ADDRESS} \
--my-bucket=${BUCKET_NAME:-coproc-0} \
--s3-endpoint=http://localhost:9000 \
--commitment-poll-interval=5s \
--commitment-timeout=60s \
--poll-interval-secs=30
