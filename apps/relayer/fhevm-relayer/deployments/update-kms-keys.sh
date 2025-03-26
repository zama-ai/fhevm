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

LOCAL_YAML="/relayer-local.yaml"
ENV_COPROCESSOR="/env.staging.coprocessor"
ENV_CONNECTOR="/env.staging.connector"

if ! docker ps -a | grep -q "generate-kms-keys"; then
    log_error "Container generate-kms-keys not found. Make sure it has been run."
    exit 1
fi

# Get logs from the container
log_info "Retrieving logs from generate-kms-keys container..."
LOGS=$(docker logs generate-kms-keys)

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

log_info "Updating $LOCAL_YAML..."
cat "$LOCAL_YAML" | sed "s|url: \"http://s3-mock:9000/kms-public/PUB/PublicKey/[^\"]*\"|url: \"$PUBLIC_KEY_URL\"|g" | \
                    sed "s|url: \"http://s3-mock:9000/kms-public/PUB/CRS/[^\"]*\"|url: \"$CRS_KEY_URL\"|g" > /tmp/local.yaml.new
cat /tmp/local.yaml.new > "$LOCAL_YAML"

log_info "Updating $ENV_COPROCESSOR..."
cat "$ENV_COPROCESSOR" | \
    sed "s|export KMS_PUBLIC_KEY=http://s3-mock:9000/kms-public/PUB/PublicKey/[^$]*|export KMS_PUBLIC_KEY=$PUBLIC_KEY_URL|g" | \
    sed "s|export KMS_SERVER_KEY=http://s3-mock:9000/kms-public/PUB/ServerKey/[^$]*|export KMS_SERVER_KEY=$SERVER_KEY_URL|g" | \
    sed "s|export KMS_SNS_KEY=http://s3-mock:9000/kms-public/PUB/SnsKey/[^$]*|export KMS_SNS_KEY=$SNS_KEY_URL|g" | \
    sed "s|export KMS_CRS_KEY=http://s3-mock:9000/kms-public/PUB/CRS/[^$]*|export KMS_CRS_KEY=$CRS_KEY_URL|g" > /tmp/env.new
cat /tmp/env.new > "$ENV_COPROCESSOR"

log_info "Updating $ENV_CONNECTOR..."
log_info "Current connector environment file content:"
cat "$ENV_CONNECTOR"

# Better pattern matching that handles both with and without quotes
cat "$ENV_CONNECTOR" | \
    sed "s|export FHE_KEY_ID=.*|export FHE_KEY_ID=\"$KEY_GEN_ID\"|g" > /tmp/env.connector.new

# Verify the change was made
if grep -q "export FHE_KEY_ID=\"$KEY_GEN_ID\"" /tmp/env.connector.new; then
    cat /tmp/env.connector.new > "$ENV_CONNECTOR"
    log_info "FHE_KEY_ID successfully updated to: $KEY_GEN_ID"
else
    log_warn "Failed to update FHE_KEY_ID. Manually verify the format in $ENV_CONNECTOR and update the script."
    log_info "The value that should be set: $KEY_GEN_ID"
fi

log_info "Configuration files updated successfully!"
log_info "Public Key ID: $KEY_GEN_ID"
log_info "CRS Key ID: $CRS_GEN_ID"