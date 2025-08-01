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

## LAYER2 - KMS_SIGNER_ADDRESS_0
BASE_URL="http://s3-mock:9000/kms-public/PUB/VerfAddress"
ENV_LAYER1="/env.staging.layer1"
ENV_LAYER2="/env.staging.layer2"
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

# LAYER1
log_info "Updating ADDRESS_KMS_SIGNER_0 in $ENV_LAYER1..."
cat "$ENV_LAYER1" | sed "s|^export ADDRESS_KMS_SIGNER_0=.*|export ADDRESS_KMS_SIGNER_0=\"$SIGNER_ADDRESS\"|g" > /tmp/env.layer1.new
if grep -q "export ADDRESS_KMS_SIGNER_0=\"$SIGNER_ADDRESS\"" /tmp/env.layer1.new; then
    cat /tmp/env.layer1.new > "$ENV_LAYER1"
    log_info "ADDRESS_KMS_SIGNER_0 successfully updated to: $SIGNER_ADDRESS in $ENV_LAYER1"
else
    log_warn "Failed to update ADDRESS_KMS_SIGNER_0. Please update manually in $ENV_LAYER1."
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

# LAYER2
log_info "Updating KMS_SIGNER_ADDRESS_0 in $ENV_LAYER2..."
cat "$ENV_LAYER2" | sed "s|^export KMS_SIGNER_ADDRESS_0=.*|export KMS_SIGNER_ADDRESS_0=\"$SIGNER_ADDRESS\"|g" > /tmp/env.layer2.new
if grep -q "export KMS_SIGNER_ADDRESS_0=\"$SIGNER_ADDRESS\"" /tmp/env.layer2.new; then
    cat /tmp/env.layer2.new > "$ENV_LAYER2"
    log_info "KMS_SIGNER_ADDRESS_0 successfully updated to: $SIGNER_ADDRESS in $ENV_LAYER2"
else
    log_warn "Failed to update KMS_SIGNER_ADDRESS_0. Please update manually in $ENV_LAYER2"
    log_info "The value that should be set: $SIGNER_ADDRESS"
fi

LOCAL_YAML="/relayer-local.yaml"
ENV_COPROCESSOR="/env.staging.coprocessor"

if ! docker ps -a | grep -q "generate-fhe-keys"; then
    log_error "Container generate-fhe-keys not found. Make sure it has been run."
    exit 1
fi

log_info "Retrieving logs from generate-fhe-keys container..."
LOGS=$(docker logs generate-fhe-keys)

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

BASE_URL="http://s3-mock:9000/kms-public/PUB"
PUBLIC_KEY_URL="$BASE_URL/PublicKey/$KEY_GEN_ID"
SERVER_KEY_URL="$BASE_URL/ServerKey/$KEY_GEN_ID"
SNS_KEY_URL="$BASE_URL/SnsKey/$KEY_GEN_ID"
CRS_KEY_URL="$BASE_URL/CRS/$CRS_GEN_ID"

## RELAYER
log_info "Updating $LOCAL_YAML..."
cat "$LOCAL_YAML" | sed "s|url: \"http://s3-mock:9000/kms-public/PUB/PublicKey/[^\"]*\"|url: \"$PUBLIC_KEY_URL\"|g" | \
                    sed "s|url: \"http://s3-mock:9000/kms-public/PUB/CRS/[^\"]*\"|url: \"$CRS_KEY_URL\"|g" > /tmp/local.yaml.new
cat /tmp/local.yaml.new > "$LOCAL_YAML"

## COPROCESSOR
log_info "Updating $ENV_COPROCESSOR..."
cat "$ENV_COPROCESSOR" | \
    sed "s|export KMS_PUBLIC_KEY=\"http://s3-mock:9000/kms-public/PUB/PublicKey/[^$]*\"|export KMS_PUBLIC_KEY=\"$PUBLIC_KEY_URL\"|g" | \
    sed "s|export KMS_SERVER_KEY=\"http://s3-mock:9000/kms-public/PUB/ServerKey/[^$]*\"|export KMS_SERVER_KEY=\"$SERVER_KEY_URL\"|g" | \
    sed "s|export KMS_SNS_KEY=\"http://s3-mock:9000/kms-public/PUB/SnsKey/[^$]*\"|export KMS_SNS_KEY=\"$SNS_KEY_URL\"|g" | \
    sed "s|export KMS_CRS_KEY=\"http://s3-mock:9000/kms-public/PUB/CRS/[^$]*\"|export KMS_CRS_KEY=\"$CRS_KEY_URL\"|g" | \
    sed "s|export FHE_KEY_ID=.*|export FHE_KEY_ID=\"$KEY_GEN_ID\"|g" > /tmp/env.coprocessor.new

# Verify all changes were made
if grep -q "export KMS_PUBLIC_KEY=\"$PUBLIC_KEY_URL\"" /tmp/env.coprocessor.new && \
   grep -q "export KMS_SERVER_KEY=\"$SERVER_KEY_URL\"" /tmp/env.coprocessor.new && \
   grep -q "export KMS_SNS_KEY=\"$SNS_KEY_URL\"" /tmp/env.coprocessor.new && \
   grep -q "export KMS_CRS_KEY=\"$CRS_KEY_URL\"" /tmp/env.coprocessor.new && \
   grep -q "export FHE_KEY_ID=\"$KEY_GEN_ID\"" /tmp/env.coprocessor.new; then
    cat /tmp/env.coprocessor.new > "$ENV_COPROCESSOR"
    log_info "All KMS keys successfully updated in coprocessor environment"
else
    log_warn "Failed to update some KMS keys in coprocessor environment. Please verify the format and update manually."
    log_info "Values that should be set:"
    log_info "KMS_PUBLIC_KEY: $PUBLIC_KEY_URL"
    log_info "KMS_SERVER_KEY: $SERVER_KEY_URL"
    log_info "KMS_SNS_KEY: $SNS_KEY_URL"
    log_info "KMS_CRS_KEY: $CRS_KEY_URL"
    log_info "FHE_KEY_ID: $KEY_GEN_ID"
fi

log_info "Configuration files updated successfully!"
log_info "Signing Key ID: $KEY_SIGNER_ID"
log_info "Public Key ID: $KEY_GEN_ID"
log_info "CRS Key ID: $CRS_GEN_ID"
