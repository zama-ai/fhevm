#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

# Shared config (DATABASE_URL, TENANT_API_KEY, AWS_*, OTEL_*) comes from the
# restored ../.env-test so every fleet service uses the same environment.
source ./../.env-test

# ------------------------------------------------------------------------------
# Environment
# ------------------------------------------------------------------------------

# .env-test defines but does not export DATABASE_URL; sns-worker reads it from
# the environment (falls back to the DATABASE_URL env var), so export it
# explicitly. AWS_* and OTEL_* are already exported by .env-test.
export DATABASE_URL

# Second argument: service name (optional)
readonly SERVICE_NAME="${2:-sns-worker-1}"
export OTEL_SERVICE_NAME="${SERVICE_NAME}"

# ------------------------------------------------------------------------------
# Feature selection
# ------------------------------------------------------------------------------

FEATURES=()

# Backward-compatible default behavior
##if [[ "${1:-}" != "default" ]]; then
##    FEATURES+=("test_decrypt_128")
##fi

# --------------------
# Parse CLI arguments
# --------------------
GPU_ENABLED=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --metrics-addr=*)
      METRICS_ADDR="${1#*=}"
      ;;
    --health-check-port=*)
      HEALTH_CHECK_PORT="${1#*=}"
      ;;
    --gpu)
      FEATURES+=("gpu")
      GPU_ENABLED=true
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
  shift
done

CARGO_FEATURES=()
if [[ ${#FEATURES[@]} -gt 0 ]]; then
    CARGO_FEATURES=(--features "$(IFS=,; echo "${FEATURES[*]}")")
fi

if [[ "$GPU_ENABLED" == true ]]; then
  source ./../.env-gpu
fi

# ------------------------------------------------------------------------------
# Diagnostics
# ------------------------------------------------------------------------------

echo "DATABASE_URL: ${DATABASE_URL}"
echo "SERVICE_NAME: ${SERVICE_NAME}"
echo "FEATURES: ${FEATURES[*]:-<none>}"

# ------------------------------------------------------------------------------
# Execution
# ------------------------------------------------------------------------------

cargo run --jobs 32 --release "${CARGO_FEATURES[@]}" -- \
    --pg-listen-channels "event_pbs_computations" "event_ciphertext_computed" \
    --pg-notify-channel "event_ciphertext128_computed" \
    --work-items-batch-size=1 \
    --pg-polling-interval=60 \
    --pg-pool-connections=10 \
    --cleanup-interval=7200s \
    --pg-auto-explain-with-min-duration=10ms \
    --bucket-name-ct64="ct64" \
    --bucket-name-ct128="ct128" \
    --schedule-policy="sequential" \
    --signer-type=private-key \
    --private-key="${TX_SENDER_PRIVATE_KEY}" \
    --health-check-port=10003
