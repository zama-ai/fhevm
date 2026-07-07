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
readonly OTEL_EXPORTER_OTLP_ENDPOINT="http://${SERVER}:4317"

export DATABASE_URL
export TENANT_API_KEY
export OTEL_EXPORTER_OTLP_ENDPOINT

# ------------------------------------------------------------------------------
# Defaults
# ------------------------------------------------------------------------------

HEALTH_CHECK_PORT=10002
FEATURES=()

# ------------------------------------------------------------------------------
# CLI parsing
# ------------------------------------------------------------------------------
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
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
  shift
done

if [[ "$GPU_ENABLED" == true ]]; then
  source ./../.env-gpu
fi

# ------------------------------------------------------------------------------
# Cargo feature assembly
# ------------------------------------------------------------------------------

CARGO_FEATURES=()
if [[ ${#FEATURES[@]} -gt 0 ]]; then
  CARGO_FEATURES=(--features "$(IFS=,; echo "${FEATURES[*]}")")
fi

# ------------------------------------------------------------------------------
# Diagnostics
# ------------------------------------------------------------------------------

echo "DATABASE_URL: ${DATABASE_URL}"
echo "HEALTH_CHECK_PORT: ${HEALTH_CHECK_PORT}"
echo "FEATURES: ${FEATURES[*]:-<none>}"

# ------------------------------------------------------------------------------
# Execution
# ------------------------------------------------------------------------------

cargo run --release "${CARGO_FEATURES[@]}" -- \
  --pg-listen-channel="event_zkpok_new_work" \
  --pg-notify-channel="event_zkpok_computed" \
  --pg-polling-interval=60 \
  --pg-pool-connections=5 \
  --worker-thread-count=4 \
  --health-check-port="${HEALTH_CHECK_PORT}"
