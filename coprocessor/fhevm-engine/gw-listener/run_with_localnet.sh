#!/bin/bash
source ./../.env-test

# ./run_with_test_env.sh --metrics-addr='0.0.0.0:9101' --health-check-port=8081

# Defaults
METRICS_ADDR="0.0.0.0:9100"
HEALTH_CHECK_PORT="8080"

# Parse CLI params
while [[ $# -gt 0 ]]; do
  case "$1" in
    --metrics-addr=*)
      METRICS_ADDR="${1#*=}"
      shift
      ;;
    --health-check-port=*)
      HEALTH_CHECK_PORT="${1#*=}"
      shift
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

echo "DATABASE_URL=$DATABASE_URL"
echo "METRICS_ADDR=$METRICS_ADDR"
echo "HEALTH_CHECK_PORT=$HEALTH_CHECK_PORT"

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
cargo run --jobs 32 --release $VERSION_OVERRIDE -- \
--database-pool-size=16 \
--verify-proof-req-database-channel="event_zkpok_new_work" \
--gw-url=${GATEWAY_WS_URL} \
--input-verification-address=${INPUT_VERIFICATION_ADDRESS} \
--error-sleep-initial-secs=1 \
--error-sleep-max-secs=10

# --kms-generation-address=${KMS_GENERATION_ADDRESS} \

