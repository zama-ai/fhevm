#!/usr/bin/env bash

set -e
set -u

# Some parameters
PATH_TO_ACCESS_KEY="${PATH_TO_ACCESS_KEY:-/minio_secrets/access_key}"
PATH_TO_SECRET_KEY="${PATH_TO_SECRET_KEY:-/minio_secrets/secret_key}"
ACCESS_KEY=$(xargs echo -n < "${PATH_TO_ACCESS_KEY}")
SECRET_KEY=$(xargs echo -n < "${PATH_TO_SECRET_KEY}")
BUCKET_NAME="${BUCKET_NAME:-kms}"
ENDPOINT="${ENDPOINT:-http://dev-s3-mock:9000}"

echo "ACCESS KEY: '${ACCESS_KEY}'"
echo "SECRET KEY: '${SECRET_KEY}'"

# Create the signing key
sign_key() {
    printf '%b' "$2" | openssl dgst -sha256 -hex -mac HMAC -macopt "key:$1" | sed 's/^.* //'
}

push_pub_resource_to_minio() {
  local RESOURCE_FOLDER_NAME=$1

  find ./keys/ -path "*/PUB*/${RESOURCE_FOLDER_NAME}/*" -type f | while read -r file; do

    FOLDER_NAME=$(echo "$file" | sed -n 's|.*\(PUB[^/]*\)/'"${RESOURCE_FOLDER_NAME}"'/.*|\1|p')
    KEY_NAME=$(basename "$file")
    FILE_NAME=$(basename "$file")
    FILE_PATH=$(realpath "$file")

    echo ""
    echo "Processing file: '$file'"
    echo "Folder name: '$FOLDER_NAME'"
    echo "Key name: '${KEY_NAME}'"
    echo "File name: '${FILE_NAME}'"
    echo "Full file path: '${FILE_PATH}'"
    echo ""

    # https://github.com/minio/wiki/wiki/how-to-use-curl-with-minio
    # Extract the host from the endpoint
    HOST=$(echo "${ENDPOINT}" | sed -e 's|^[^/]*//||' -e 's|/.*$||')
    resource="/${BUCKET_NAME}/${FOLDER_NAME}/${RESOURCE_FOLDER_NAME}/${FILE_NAME}"
    content_type="application/octet-stream"
    date=$(TZ=UTC date -R)
    _signature="PUT\n\n${content_type}\n${date}\n${resource}"
    echo "'$_signature'"
    signature=$(printf '%b' "${_signature}" | openssl sha1 -hmac "${SECRET_KEY}" -binary | base64)
    echo "'$signature'"
    curl -v --fail-with-body -X PUT -T "${FILE_PATH}" \
              -H "Host: $HOST" \
              -H "Date: ${date}" \
              -H "Content-Type: ${content_type}" \
              -H "Authorization: AWS ${ACCESS_KEY}:${signature}" \
              "http://${HOST}${resource}"
  done
}

push_pub_resource_to_minio "PublicKey"
push_pub_resource_to_minio "PublicKeyMetadata"
push_pub_resource_to_minio "ServerKey"
push_pub_resource_to_minio "CRS"
push_pub_resource_to_minio "VerfAddress"
push_pub_resource_to_minio "VerfKey"
