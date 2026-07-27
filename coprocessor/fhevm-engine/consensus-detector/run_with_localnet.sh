#!/bin/bash
set -euo pipefail
source ./../.env-test

# The detector resolves each operator's S3 bucket URL from the on-chain
# GatewayConfig, which the e2e sets to `http://minio:9000/ct128`. That hostname
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
cargo run --release -- \
--database-url=${DATABASE_URL} \
--database-pool-size=4 \
--gw-url=${GATEWAY_WS_URL} \
--gateway-config-address=${GATEWAY_CONFIG_ADDRESS} \
--my-bucket=${BUCKET_NAME_CT128:-ct128} \
--s3-endpoint=http://localhost:9000 \
--commitment-poll-interval=5s \
--commitment-timeout=60s \
--poll-interval-secs=30
