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

## GATEWAY - KMS_SIGNER_ADDRESS_0
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_URL="http://localhost:9000/kms-public/PUB/VerfAddress"
ENV_HOST="${SCRIPT_DIR}/../env/staging/.env.host.local"
ENV_GATEWAY="${SCRIPT_DIR}/../env/staging/.env.gateway.local"
ENV_RELAYER="${SCRIPT_DIR}/../env/staging/.env.relayer.local"
LOCAL_YAML="${SCRIPT_DIR}/../config/relayer/local.yaml.local"
KEY_SIGNER_ID=$(docker logs kms-core | grep "Successfully stored public server signing key under the handle" | sed 's/.*handle \([^ ]*\).*/\1/')
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

# HOST
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_HOST..."
cat $ENV_HOST | sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" > /tmp/env.host.new
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" /tmp/env.host.new; then
    cat /tmp/env.host.new > $ENV_HOST
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_HOST"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_HOST."
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

# GATEWAY
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_GATEWAY..."
cat $ENV_GATEWAY | sed "s|^KMS_SIGNER_ADDRESS_0=.*|KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS|g" > /tmp/env.gateway.new
if grep -q "KMS_SIGNER_ADDRESS_0=$SIGNER_ADDRESS" /tmp/env.gateway.new; then
    cat /tmp/env.gateway.new > $ENV_GATEWAY
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_GATEWAY"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_GATEWAY"
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

if ! docker ps -a | grep -q "fhevm-generate-fhe-keys"; then
    log_error "Container fhevm-generate-fhe-keys not found. Make sure it has been run."
    exit 1
fi

log_info "Retrieving logs from fhevm-generate-fhe-keys container..."
LOGS=$(docker logs fhevm-generate-fhe-keys)

# Extract key request IDs
KEY_GEN_ID=$(echo "$LOGS" | grep -A1 "insecure keygen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')
CRS_GEN_ID=$(echo "$LOGS" | grep -A1 "crsgen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')

if [ -z "$KEY_GEN_ID" ] || [ -z "$CRS_GEN_ID" ]; then
    log_error "Failed to extract key IDs from logs."
    log_info "Log content:"
    echo "$LOGS"
    exit 1
fi

log_info "Extracted key IDs:"
log_info "Key Gen ID: $KEY_GEN_ID"
log_info "CRS Gen ID: $CRS_GEN_ID"

BASE_URL="http://minio:9000/kms-public/PUB"
PUBLIC_KEY_URL="$BASE_URL/PublicKey/$KEY_GEN_ID"
SERVER_KEY_URL="$BASE_URL/ServerKey/$KEY_GEN_ID"
SNS_KEY_URL="$BASE_URL/SnsKey/$KEY_GEN_ID"
CRS_KEY_URL="$BASE_URL/CRS/$CRS_GEN_ID"

## RELAYER
log_info "Updating $ENV_RELAYER..."
log_info "PUBLIC_KEY_URL: $PUBLIC_KEY_URL"
log_info "CRS_KEY_URL: $CRS_KEY_URL"
cat "$ENV_RELAYER" | \
    sed "s|^APP_KEYURL__FHE_PUBLIC_KEY__URL=.*|APP_KEYURL__FHE_PUBLIC_KEY__URL=$PUBLIC_KEY_URL|g" | \
    sed "s|^APP_KEYURL__CRS__URL=.*|APP_KEYURL__CRS__URL=$CRS_KEY_URL|g" > /tmp/env.relayer.new

# Verify all changes were made
if grep -q "APP_KEYURL__FHE_PUBLIC_KEY__URL=$PUBLIC_KEY_URL" /tmp/env.relayer.new && \
   grep -q "APP_KEYURL__CRS__URL=$CRS_KEY_URL" /tmp/env.relayer.new; then
    cat /tmp/env.relayer.new > "$ENV_RELAYER"
    log_info "KMS keys successfully updated in $ENV_RELAYER"
else
    log_warn "Failed to update some KMS keys in relayer environment. Please verify the format and update manually."
    log_info "Values that should be set:"
    log_info "APP_KEYURL__FHE_PUBLIC_KEY__URL: $PUBLIC_KEY_URL"
    log_info "APP_KEYURL__CRS__URL: $CRS_KEY_URL"
fi

## COPROCESSORS
for i in {0..2}
do
    ENV_COPROCESSOR="${SCRIPT_DIR}/../env/staging/.env.coprocessor-${i}.local"
    if [ ! -f "$ENV_COPROCESSOR" ]; then
        log_warn "Coprocessor env file not found, skipping: $ENV_COPROCESSOR"
        continue
    fi

    log_info "Updating $ENV_COPROCESSOR..."
    TEMP_FILE="/tmp/env.coprocessor.${i}.new"
    cat "$ENV_COPROCESSOR" | \
        sed "s|KMS_PUBLIC_KEY=http://minio:9000/kms-public/PUB/PublicKey/[^$]*|KMS_PUBLIC_KEY=$PUBLIC_KEY_URL|g" | \
        sed "s|KMS_SERVER_KEY=http://minio:9000/kms-public/PUB/ServerKey/[^$]*|KMS_SERVER_KEY=$SERVER_KEY_URL|g" | \
        sed "s|KMS_SNS_KEY=http://minio:9000/kms-public/PUB/SnsKey/[^$]*|KMS_SNS_KEY=$SNS_KEY_URL|g" | \
        sed "s|KMS_CRS_KEY=http://minio:9000/kms-public/PUB/CRS/[^$]*|KMS_CRS_KEY=$CRS_KEY_URL|g" | \
        sed "s|FHE_KEY_ID=.*|FHE_KEY_ID=$KEY_GEN_ID|g" > "$TEMP_FILE"

    # Verify all changes were made
    if grep -q "KMS_PUBLIC_KEY=$PUBLIC_KEY_URL" "$TEMP_FILE" && \
       grep -q "KMS_SERVER_KEY=$SERVER_KEY_URL" "$TEMP_FILE" && \
       grep -q "KMS_SNS_KEY=$SNS_KEY_URL" "$TEMP_FILE" && \
       grep -q "KMS_CRS_KEY=$CRS_KEY_URL" "$TEMP_FILE" && \
       grep -q "FHE_KEY_ID=$KEY_GEN_ID" "$TEMP_FILE"; then
        cat "$TEMP_FILE" > "$ENV_COPROCESSOR"
        log_info "KMS keys successfully updated in $ENV_COPROCESSOR"
    else
        log_warn "Failed to update some KMS keys in coprocessor environment. Please verify the format and update manually: $ENV_COPROCESSOR"
        log_info "Values that should be set:"
        log_info "KMS_PUBLIC_KEY: $PUBLIC_KEY_URL"
        log_info "KMS_SERVER_KEY: $SERVER_KEY_URL"
        log_info "KMS_SNS_KEY: $SNS_KEY_URL"
        log_info "KMS_CRS_KEY: $CRS_KEY_URL"
        log_info "FHE_KEY_ID: $KEY_GEN_ID"
    fi
done

log_info "Configuration files updated successfully!"
log_info "Signing Key ID: $KEY_SIGNER_ID"
log_info "Public Key ID: $KEY_GEN_ID"
log_info "CRS Key ID: $CRS_GEN_ID"