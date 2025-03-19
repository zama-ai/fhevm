#!/bin/bash

set -e

# Install required dependencies
apt update && apt install -y wget

# Configuration
MINIO_URL="http://s3-mock:9000"
AWS_ACCESS_KEY_ID="httpz-access-key"
AWS_SECRET_ACCESS_KEY="httpz-access-secret-key"
BUCKET_NAME="kms-public"
PATHS=(PublicKey ServerKey SnsKey CRS)
FILES_DIR="/keys"
IDS=(
    "408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
    "408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
    "408d8cbaa51dece7f782fe04ba0b1c1d017b10880c538b7c72037468fe5c97ee"
    "a5fedad3fd734a598fb67452099229445cb68447198fb56f29bb64d98953d002"
)

# Install Minio client
wget https://dl.min.io/client/mc/release/linux-amd64/mc -O /usr/local/bin/mc
chmod +x /usr/local/bin/mc

# Configure Minio client
mc alias set minio "$MINIO_URL" "$AWS_ACCESS_KEY_ID" "$AWS_SECRET_ACCESS_KEY"

# Upload files
for i in "${!IDS[@]}"; do
    id="${IDS[$i]}"
    path="${PATHS[$i]}"
    file="$FILES_DIR/$path/$id"

    if [ ! -f "$file" ]; then
        echo "Error: File $file not found"
        exit 1
    fi
            
    DEST_PATH="minio/${BUCKET_NAME}/kms/PUB/${path}/${id}"
    
    # Copy with file preservation and checksum
    if ! mc cp \
        --attr "Content-Type=application/octet-stream" \
        --preserve \
        --checksum SHA256 \
        "$file" \
        "$DEST_PATH"; then
        echo "Error: Failed to upload $file"
        exit 1
    fi
            
    # Verify upload
    if ! mc ls "$DEST_PATH"; then
        echo "Error: Failed to verify upload"
        exit 1
    fi
            
    echo "Successfully uploaded $file to $DEST_PATH"
done