#!/bin/bash

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Get and setup the Key signer ID (used as KMS_SIGNER_ADDRESS_0 at Gateway and Host)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ENV_HOST="${SCRIPT_DIR}/../env/staging/.env.host-sc.local"
ENV_GATEWAY="${SCRIPT_DIR}/../env/staging/.env.gateway-sc.local"
extract_signing_key_id() {
    docker logs kms-core 2>&1 \
        | sed -nE 's/.*under the handle ([A-Fa-f0-9]{64}).*/\1/p; s/.*Signing Key ID[: ]+([A-Fa-f0-9]{64}).*/\1/p; s|.*VerfKey/([A-Fa-f0-9]{64}).*|\1|p' \
        | tail -n1
}

extract_signer_address_from_logs() {
    docker logs kms-core 2>&1 \
        | sed -nE 's/.*Successfully stored ethereum address (0x[a-fA-F0-9]{40}) under the handle [A-Fa-f0-9]{64}.*/\1/p; s/.*ethereum address[: ]+(0x[a-fA-F0-9]{40}).*/\1/p' \
        | tail -n1
}

KEY_SIGNER_ID=""
MAX_KEY_ID_ATTEMPTS=30
for attempt in $(seq 1 "$MAX_KEY_ID_ATTEMPTS"); do
    KEY_SIGNER_ID="$(extract_signing_key_id)"
    if [ -n "$KEY_SIGNER_ID" ]; then
        break
    fi
    if [ "$attempt" -lt "$MAX_KEY_ID_ATTEMPTS" ]; then
        log_warn "Signing key ID not found in kms-core logs yet, retrying in 2s... ($attempt/$MAX_KEY_ID_ATTEMPTS)"
        sleep 2
    fi
done

if [ -z "$KEY_SIGNER_ID" ]; then
    log_error "Failed to extract signing key ID from kms-core logs after retries."
    exit 1
fi

log_info "Signing Key ID: $KEY_SIGNER_ID"

SIGNER_ADDRESS=""

# Prefer runtime signer address from kms-core logs. This avoids stale MinIO signer
# objects when kms-core rotates keys but refuses to overwrite existing S3 entries.
MAX_SIGNER_LOG_ATTEMPTS=30
for attempt in $(seq 1 "$MAX_SIGNER_LOG_ATTEMPTS"); do
    SIGNER_ADDRESS="$(extract_signer_address_from_logs)"
    if [ -n "$SIGNER_ADDRESS" ]; then
        log_info "Retrieved KMS signer address from kms-core logs"
        break
    fi
    if [ "$attempt" -lt "$MAX_SIGNER_LOG_ATTEMPTS" ]; then
        log_warn "KMS signer address not found in kms-core logs yet, retrying in 2s... ($attempt/$MAX_SIGNER_LOG_ATTEMPTS)"
        sleep 2
    fi
done

if [ -z "$SIGNER_ADDRESS" ]; then
    MAX_SIGNER_ADDRESS_ATTEMPTS=30
    for attempt in $(seq 1 "$MAX_SIGNER_ADDRESS_ATTEMPTS"); do
        for base_path in "PUB/PUB/VerfAddress" "PUB/VerfAddress"; do
            SIGNER_ADDRESS_URL="http://localhost:9000/kms-public/${base_path}/${KEY_SIGNER_ID}"
            if SIGNER_ADDRESS=$(curl -sSf "$SIGNER_ADDRESS_URL" 2>/dev/null); then
                log_info "Retrieved KMS signer address from $SIGNER_ADDRESS_URL"
                break 2
            fi
        done
        if [ "$attempt" -lt "$MAX_SIGNER_ADDRESS_ATTEMPTS" ]; then
            log_warn "KMS signer address not available in MinIO yet, retrying in 2s... ($attempt/$MAX_SIGNER_ADDRESS_ATTEMPTS)"
            sleep 2
        fi
    done
fi

if [ -z "$SIGNER_ADDRESS" ]; then
    log_error "Failed to retrieve KMS signer address from MinIO for key handle: $KEY_SIGNER_ID"
    exit 1
fi

# Validate the address format (should be a hex address)
if [[ ! "$SIGNER_ADDRESS" =~ ^0x[a-fA-F0-9]{40}$ ]]; then
    log_warn "Retrieved signer address doesn't match expected format: $SIGNER_ADDRESS"
    log_info "Using the address anyway, please verify manually."
fi

TMP_HOST_ENV="$(mktemp)"
TMP_GATEWAY_ENV="$(mktemp)"
trap 'rm -f "$TMP_HOST_ENV" "$TMP_GATEWAY_ENV"' EXIT

# Setup KMS_SIGNER_ADDRESS_0 for Host contracts
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_HOST..."
sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" "$ENV_HOST" > "$TMP_HOST_ENV"
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" "$TMP_HOST_ENV"; then
    cat "$TMP_HOST_ENV" > "$ENV_HOST"
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_HOST"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_HOST."
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

# Setup KMS_SIGNER_ADDRESS_0 for Gateway contracts
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_GATEWAY..."
sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" "$ENV_GATEWAY" > "$TMP_GATEWAY_ENV"
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" "$TMP_GATEWAY_ENV"; then
    cat "$TMP_GATEWAY_ENV" > "$ENV_GATEWAY"
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_GATEWAY"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_GATEWAY"
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

log_info "KMS signer address configuration files updated successfully!"
log_info "Signing Key ID: $KEY_SIGNER_ID"
