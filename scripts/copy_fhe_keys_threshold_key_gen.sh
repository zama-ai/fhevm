#!/usr/bin/env bash

# Script to create keys by downloading from MinIO and copying them to the appropriate folder
# Usage: ./caller_script.sh [LOCAL_BUILD_PUBLIC_KEY_PATH]

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

# File paths in MinIO
FILES_TO_DOWNLOAD=(
  "PUB-p1/PublicKey/d4d17a412a6533599b010c8ffc3d6ebdc6b1cfad"
  "PUB-p1/ServerKey/d4d17a412a6533599b010c8ffc3d6ebdc6b1cfad"
  "PUB-p1/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906"
  "PUB-p1/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
  "PUB-p2/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
  "PUB-p3/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
  "PUB-p4/VerfAddress/e164d9de0bec6656928726433cc56bef6ee8417a"
)

echo "###########################################################"
echo "All the required keys have been downloaded to $KEYS_FULL_PATH"
echo "###########################################################"

# Copy the required files to the specified public path
echo "Copying keys to $NETWORK_KEYS_PUBLIC_PATH..."


./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[0]}" "$NETWORK_KEYS_PUBLIC_PATH/sks"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[1]}" "$NETWORK_KEYS_PUBLIC_PATH/pks"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[2]}" "$NETWORK_KEYS_PUBLIC_PATH/pp"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[3]}" "$NETWORK_KEYS_PUBLIC_PATH/signer1"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[4]}" "$NETWORK_KEYS_PUBLIC_PATH/signer2"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[5]}" "$NETWORK_KEYS_PUBLIC_PATH/signer3"
./scripts/download_from_minio.sh "localhost:9000" "$MINIO_ROOT_USER" "$MINIO_ROOT_PASSWORD" "kms" "${FILES_TO_DOWNLOAD[6]}" "$NETWORK_KEYS_PUBLIC_PATH/signer4"

echo "###########################################################"
echo "All keys have been copied to $NETWORK_KEYS_PUBLIC_PATH"
echo "###########################################################"
