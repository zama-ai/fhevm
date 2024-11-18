#!/bin/sh

# Script to create keys by downloading from MinIO and copying them to the appropriate folder
# Usage: ./copy_fhe_keys_threshold_key_gen.sh [LOCAL_BUILD_PUBLIC_KEY_PATH]

set -Eeuo pipefail

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
    echo "Error: Failed to get keys from gateway at http://localhost:7077/keyurl. Is gateway running?"
    exit 1
fi

# Get the URLs and extract the IDs
PKS_URL=$(jq -r '.response.fhe_key_info[0].fhe_public_key.urls[0]' <<< "$KEYS_URLS_JSON")
SKS_URL=$(jq -r '.response.fhe_key_info[0].fhe_server_key.urls[0]' <<< "$KEYS_URLS_JSON")
CRS_URL=$(jq -r '.response.crs."2048".urls[0]' <<< "$KEYS_URLS_JSON")
SIGNER1_URL=$(jq -r '.response.verf_public_key[0].verf_public_key_address' <<< "$KEYS_URLS_JSON")

# Extract only the ID part from each URL
PKS_ID=$(basename "$PKS_URL")
SKS_ID=$(basename "$SKS_URL")
CRS_ID=$(basename "$CRS_URL")
SIGNER1_ID=$(basename "$SIGNER1_URL")

# Prepare the list of files to download
FILES_TO_DOWNLOAD=(
  "PUB/PublicKey/$PKS_ID"
  "PUB/ServerKey/$SKS_ID"
  "PUB/CRS/$CRS_ID"
  "PUB/VerfAddress/$SIGNER1_ID"
)

# Print the file paths for confirmation
echo "Files to download:"
for path in "${FILES_TO_DOWNLOAD[@]}"; do
    echo "$path"
done

echo "###########################################################"
echo "All the required keys will be downloaded to $KEYS_FULL_PATH"
echo "###########################################################"

# Copy the required files to the specified public path
echo "Copying keys to $NETWORK_KEYS_PUBLIC_PATH..."

curl -o "$NETWORK_KEYS_PUBLIC_PATH/pks" "http://localhost:9000/kms/${FILES_TO_DOWNLOAD[0]}"
curl -o "$NETWORK_KEYS_PUBLIC_PATH/sks" "http://localhost:9000/kms/${FILES_TO_DOWNLOAD[1]}"
curl -o "$NETWORK_KEYS_PUBLIC_PATH/pp"  "http://localhost:9000/kms/${FILES_TO_DOWNLOAD[2]}"
curl -o "$NETWORK_KEYS_PUBLIC_PATH/signer1" "http://localhost:9000/kms/${FILES_TO_DOWNLOAD[3]}"

echo "###########################################################"
echo "All keys have been copied to $NETWORK_KEYS_PUBLIC_PATH"
echo "###########################################################"
