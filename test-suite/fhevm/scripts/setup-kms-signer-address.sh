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
BASE_URL="http://localhost:9000/kms-public/PUB/VerfAddress"
ENV_HOST="${SCRIPT_DIR}/../env/staging/.env.host-sc.local"
ENV_GATEWAY="${SCRIPT_DIR}/../env/staging/.env.gateway-sc.local"
KEY_SIGNER_ID=$(docker logs kms-core | grep "Successfully stored public centralized server signing key under the handle" | sed 's/.*handle \([^ ]*\).*/\1/')
SIGNER_ADDRESS_URL="$BASE_URL/$KEY_SIGNER_ID"

if [ -z "$KEY_SIGNER_ID" ]; then
    log_error "Failed to extract signing key ID from logs."
    exit 1
fi

log_info "Signing Key ID: $KEY_SIGNER_ID"
log_info "Retrieving KMS signer address from $SIGNER_ADDRESS_URL"
curl -O "$SIGNER_ADDRESS_URL"
SIGNER_ADDRESS=$(cat "$KEY_SIGNER_ID")

# Validate the address format (should be a hex address)
if [[ ! "$SIGNER_ADDRESS" =~ ^0x[a-fA-F0-9]{40}$ ]]; then
    log_warn "Retrieved signer address doesn't match expected format: $SIGNER_ADDRESS"
    log_info "Using the address anyway, please verify manually."
fi

# Setup KMS_SIGNER_ADDRESS_0 for Host contracts
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_HOST..."
cat $ENV_HOST | sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" > /tmp/env.host-sc.new
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" /tmp/env.host-sc.new; then
    cat /tmp/env.host-sc.new > $ENV_HOST
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_HOST"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_HOST."
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

# Setup KMS_SIGNER_ADDRESS_0 for Gateway contracts
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_GATEWAY..."
cat $ENV_GATEWAY | sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" > /tmp/env.gateway-sc.new
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" /tmp/env.gateway-sc.new; then
    cat /tmp/env.gateway-sc.new > $ENV_GATEWAY
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_GATEWAY"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_GATEWAY"
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

log_info "KMS signer address configuration files updated successfully!"
log_info "Signing Key ID: $KEY_SIGNER_ID"
