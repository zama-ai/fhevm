#!/usr/bin/env bash

# Script to create keys by downloading from MinIO and copying them to the appropriate folder
# Usage: ./copy_fhe_keys_threshold_key_gen.sh [LOCAL_BUILD_PUBLIC_KEY_PATH]

set -Eeuo pipefail

# Load environment variables from .env file
[ -f .env ] && source .env || {
  echo ".env file not found!"
  exit 1
}

# Ensure the required environment variables are present
[ -z "$MINIO_ROOT_USER" ] && {
  echo "MINIO_ROOT_USER is not set. Please set it in the .env file."
  exit 1
}

[ -z "$MINIO_ROOT_PASSWORD" ] && {
  echo "MINIO_ROOT_PASSWORD is not set. Please set it in the .env file."
  exit 1
}

CURRENT_FOLDER=$PWD
KEYS_FULL_PATH="$CURRENT_FOLDER/res/keys"
mkdir -p "$KEYS_FULL_PATH"

if [ "$#" -ge 1 ]; then
    LOCAL_BUILD_PUBLIC_KEY_PATH=$1
    NETWORK_KEYS_PUBLIC_PATH="${LOCAL_BUILD_PUBLIC_KEY_PATH}"
else
    NETWORK_KEYS_PUBLIC_PATH="./volumes/network-public-fhe-keys"
fi

mkdir -p "$NETWORK_KEYS_PUBLIC_PATH"

# Get all the keys' info from the gateway
if ! KEYS_URLS_JSON=$(curl -f http://localhost:7077/keyurl); then
    echo "Error: Failed to get keys from gateway at http://localhost:7077/keyurl. Is gateway running ?"
    exit 1
fi

# Get the keys' urls and extract the ID at the end
PKS_URL=$(jq -r '.response.fhe_key_info[0].fhe_public_key.urls[0]' <<< "$KEYS_URLS_JSON")
SKS_URL=$(jq -r '.response.fhe_key_info[0].fhe_server_key.urls[0]' <<< "$KEYS_URLS_JSON")
CRS_URL=$(jq -r '.response.crs."2048".urls[0]' <<< "$KEYS_URLS_JSON")
SIGNER1_URL=$(jq -r '.response.verf_public_key[0].verf_public_key_address' <<< "$KEYS_URLS_JSON")
SIGNER2_URL=$(jq -r '.response.verf_public_key[1].verf_public_key_address' <<< "$KEYS_URLS_JSON")
SIGNER3_URL=$(jq -r '.response.verf_public_key[2].verf_public_key_address' <<< "$KEYS_URLS_JSON")
SIGNER4_URL=$(jq -r '.response.verf_public_key[3].verf_public_key_address' <<< "$KEYS_URLS_JSON")

# Extract only the ID part from each URL
PKS_ID=$(basename "$PKS_URL")
SKS_ID=$(basename "$SKS_URL")
CRS_ID=$(basename "$CRS_URL")
SIGNER1_ID=$(basename "$SIGNER1_URL")
SIGNER2_ID=$(basename "$SIGNER2_URL")
SIGNER3_ID=$(basename "$SIGNER3_URL")
SIGNER4_ID=$(basename "$SIGNER4_URL")

# Print the IDs
echo $PKS_ID
echo $SKS_ID
echo $CRS_ID
echo $SIGNER1_ID
echo $SIGNER2_ID
echo $SIGNER3_ID
echo $SIGNER4_ID

# Update the array for file download paths
FILES_TO_DOWNLOAD=(
  "PUB-p1/PublicKey/$PKS_ID"
  "PUB-p1/ServerKey/$SKS_ID"
  "PUB-p1/CRS/$CRS_ID"
  "PUB-p1/VerfAddress/$SIGNER1_ID"
  "PUB-p2/VerfAddress/$SIGNER2_ID"
  "PUB-p3/VerfAddress/$SIGNER3_ID"
  "PUB-p4/VerfAddress/$SIGNER4_ID"
)

# Print the file paths for confirmation
echo "Files to download:"
for path in "${FILES_TO_DOWNLOAD[@]}"; do
    echo "$path"
  done

echo "###########################################################"
echo "All the required keys have been downloaded to $KEYS_FULL_PATH"
echo "###########################################################"

# Copy the required files to the specified public path
echo "Copying keys to $NETWORK_KEYS_PUBLIC_PATH..."


./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[0]}" "$NETWORK_KEYS_PUBLIC_PATH/pks"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[1]}" "$NETWORK_KEYS_PUBLIC_PATH/sks"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[2]}" "$NETWORK_KEYS_PUBLIC_PATH/pp"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[3]}" "$NETWORK_KEYS_PUBLIC_PATH/signer1"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[4]}" "$NETWORK_KEYS_PUBLIC_PATH/signer2"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[5]}" "$NETWORK_KEYS_PUBLIC_PATH/signer3"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[6]}" "$NETWORK_KEYS_PUBLIC_PATH/signer4"

echo "###########################################################"
echo "All keys have been copied to $NETWORK_KEYS_PUBLIC_PATH"
echo "###########################################################"
