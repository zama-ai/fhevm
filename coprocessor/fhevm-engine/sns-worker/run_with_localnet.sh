#!/usr/bin/env bash
set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

readonly POSTGRES_USER="postgres"
readonly POSTGRES_PASSWORD="postgres"
readonly SERVER="0.0.0.0"

readonly DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${SERVER}:5432/coprocessor"
readonly TENANT_API_KEY="a1503fb6-d79b-4e9e-826d-44cf262f3e05"

readonly AWS_ACCESS_KEY_ID="fhevm-access-key"
readonly AWS_SECRET_ACCESS_KEY="fhevm-access-secret-key"
readonly AWS_ENDPOINT_URL="http://${SERVER}:9000"
readonly AWS_REGION="eu-west-1"

readonly OTEL_EXPORTER_OTLP_ENDPOINT="http://${SERVER}:7717"

# ------------------------------------------------------------------------------
# Environment
# ------------------------------------------------------------------------------

export DATABASE_URL
export TENANT_API_KEY
export AWS_ACCESS_KEY_ID
export AWS_SECRET_ACCESS_KEY
export AWS_ENDPOINT_URL
export AWS_REGION
export OTEL_EXPORTER_OTLP_ENDPOINT

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
    --tenant-api-key="${TENANT_API_KEY}" \
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
    --health-check-port=10003
